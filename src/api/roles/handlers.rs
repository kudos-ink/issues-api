use bytes::Buf;
use log::{error, info, warn};
use warp::{
    http::StatusCode,
    reject,
    reject::Rejection,
    reply::{json, with_status, Reply},
};

use crate::{
    api::{projects::db::DBProject, roles::{models::{KudosRole, NewRole}, utils::user_has_at_least_one_role}, users::db::DBUser}, middlewares::github::model::GitHubUser, types::{PaginatedResponse, PaginationParams}
};

use super::{
    db::DBRole,
    errors::RoleError,
    models::{NewUserProjectRole, UpdateRole},
};

pub async fn by_id(id: i32, db_access: impl DBRole) -> Result<impl Reply, Rejection> {
    info!("getting role '{id}'");
    match db_access.by_id(id)? {
        None => Err(warp::reject::custom(RoleError::NotFound(id)))?,
        Some(repository) => Ok(json(&repository)),
    }
}

pub async fn all_handler(
    db_access: impl DBRole,
    pagination: PaginationParams,
) -> Result<impl Reply, Rejection> {
    info!("getting all the roles");
    let (roles, total_count) = db_access.all(pagination.clone())?;
    let has_next_page = pagination.offset + pagination.limit < total_count;
    let has_previous_page = pagination.offset > 0;

    let response = PaginatedResponse {
        total_count: Some(total_count),
        has_next_page,
        has_previous_page,
        data: roles,
    };

    Ok(json(&response))
}

pub async fn create_handler(
    user: GitHubUser,
    buf: impl Buf,
    db_access: impl DBRole,
) -> Result<impl Reply, Rejection> {
    let user_roles = DBRole::user_roles(&db_access, &user.username)?;
    user_has_at_least_one_role(
        user_roles.clone(),
        vec![
            KudosRole::Admin,
        ],
    )?;
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let role: NewRole = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid role '{e}'",);
        reject::custom(RoleError::InvalidPayload(e))
    })?;

    info!("creating role '{}'", role.name);
    match DBRole::create(&db_access, &role) {
        Ok(role) => {
            info!("role id '{}' created", role.id);
            Ok(with_status(json(&role), StatusCode::CREATED))
        }
        Err(err) => {
            error!("error creating the role '{:?}': {}", role, err);
            Err(warp::reject::custom(RoleError::CannotCreate(
                "error creating the role".to_owned(),
            )))
        }
    }
}
pub async fn update_handler(
    id: i32,
    user: GitHubUser,
    buf: impl Buf,
    db_access: impl DBRole,
) -> Result<impl Reply, Rejection> {
    let user_roles = DBRole::user_roles(&db_access, &user.username)?;
    user_has_at_least_one_role(
        user_roles.clone(),
        vec![
            KudosRole::Admin,
        ],
    )?;
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let role: UpdateRole = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid role update: '{e}'",);
        reject::custom(RoleError::InvalidPayload(e))
    })?;

    match DBRole::by_id(&db_access, id)? {
        Some(p) => match db_access.update(p.id, &role) {
            Ok(role) => {
                info!("role '{}' updated", role.id);
                Ok(with_status(json(&role), StatusCode::OK))
            }
            Err(error) => {
                error!("error updating the role '{:?}': {}", role, error);

                Err(warp::reject::custom(RoleError::CannotUpdate(
                    "error updating the role".to_owned(),
                )))
            }
        },
        None => Err(warp::reject::custom(RoleError::NotFound(id))),
    }
}
pub async fn delete_handler(
    id: i32,
    user: GitHubUser,
    db_access: impl DBRole,
    ) -> Result<impl Reply, Rejection> {
        let user_roles = DBRole::user_roles(&db_access, &user.username)?;
        user_has_at_least_one_role(
            user_roles.clone(),
            vec![
                KudosRole::Admin,
            ],
        )?;
    match DBRole::by_id(&db_access, id)? {
        Some(_) => {
            let _ = &db_access.delete(id)?;
            Ok(StatusCode::NO_CONTENT)
        }
        None => Err(warp::reject::custom(RoleError::NotFound(id)))?,
    }
}

pub async fn create_role_to_user_and_project(
    user: GitHubUser,
    buf: impl Buf,
    db_access: impl DBRole + DBUser + DBProject,
) -> Result<impl Reply, Rejection> {
    let user_roles = DBRole::user_roles(&db_access, &user.username)?;
    user_has_at_least_one_role(
        user_roles.clone(),
        vec![
            KudosRole::Admin,
        ],
    )?;
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let user_project_role: NewUserProjectRole =
        serde_path_to_error::deserialize(des).map_err(|e| {
            let e = e.to_string();
            warn!("invalid user role project assignee: '{e}'",);
            reject::custom(RoleError::InvalidPayload(e))
        })?;
        // TODO: improve project_id verification
    match DBUser::by_id(&db_access, user_project_role.user_id)? {
        Some(_) => match DBRole::by_id(&db_access, user_project_role.role_id)? {
            // Some(_) => match DBProject::by_id(&db_access, user_project_role.project_id)? {
                Some(_) => {
                    match DBRole::create_role_to_user_and_project(&db_access, &user_project_role) {
                        Ok(user_project_role) => {
                            info!("assignation '{}' created", user_project_role.id);
                            Ok(with_status(json(&user_project_role), StatusCode::CREATED))
                        }
                        Err(error) => {
                            error!(
                                "error creating the assignation '{:?}': {}",
                                user_project_role, error
                            );
                            if error.to_string().contains("unique_user_project_role") {
                                Err(warp::reject::custom(RoleError::AssignationAlreadyExists()))
                            } else {
                                Err(warp::reject::custom(RoleError::CannotCreate(
                                    "error creating assignation".to_owned(),
                                )))
                            }
                        }
                    }
                // }
                // None => Err(warp::reject::custom(RoleError::ProjectNotFound(
                //     user_project_role.project_id,
                // ))),
            },
            None => Err(warp::reject::custom(RoleError::RoleNotFound(
                user_project_role.role_id,
            ))),
        },
        None => Err(warp::reject::custom(RoleError::UserNotFound(
            user_project_role.user_id,
        ))),
    }
}
pub async fn delete_role_to_user_and_project(
    id: i32,
    user: GitHubUser,
    db_access: impl DBRole,
) -> Result<impl Reply, Rejection> {
    let user_roles = DBRole::user_roles(&db_access, &user.username)?;
    user_has_at_least_one_role(
        user_roles.clone(),
        vec![
            KudosRole::Admin,
        ],
    )?;
    match db_access.delete_role_to_user_and_project(id) {
        Ok(role) => {
            info!("assignation '{}' deleted", id);
            Ok(with_status(json(&role), StatusCode::NO_CONTENT))
        }
        Err(error) => {
            error!("error deleting the assignation '{id}': {error}");
            Err(warp::reject::custom(RoleError::CannotDelete(
                "error deleting the assignation".to_owned(),
            )))
        }
    }
}
