use crate::{api::roles::{db::DBRole, models::KudosRole, utils::user_has_at_least_one_role}, middlewares::github::model::GitHubUser, types::{PaginatedResponse, PaginationParams}};

use super::{
    db::DBProject,
    errors::ProjectError,
    models::{NewProject, QueryParams, UpdateProject},
};
use bytes::Buf;
use log::{error, info, warn};
use warp::{
    http::StatusCode,
    reject,
    reject::Rejection,
    reply::{json, with_status, Reply},
};

pub async fn all_handler(
    db_access: impl DBProject,
    params: QueryParams,
    pagination: PaginationParams,
) -> Result<impl Reply, Rejection> {
    let (projects, total_count) = db_access.all(params, pagination.clone())?;
    let has_next_page = pagination.offset + pagination.limit < total_count;
    let has_previous_page = pagination.offset > 0;

    let response = PaginatedResponse {
        total_count: Some(total_count),
        has_next_page,
        has_previous_page,
        data: projects,
    };

    Ok(json(&response))
}

pub async fn options(
    db_access: impl DBProject,
    params: QueryParams,
) -> Result<impl Reply, Rejection> {
    let options = db_access.options(params)?;
    Ok(json(&options))
}

pub async fn create_handler(
    user: GitHubUser,
    buf: impl Buf,
    db_access: impl DBProject + DBRole,
) -> Result<impl Reply, Rejection> {
    let user_roles = DBRole::user_roles(&db_access, &user.username)?;
    user_has_at_least_one_role(
        user_roles.clone(),
        vec![
            KudosRole::Admin,
        ],
    )?;
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let project: NewProject = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid project '{e}'",);
        reject::custom(ProjectError::InvalidPayload(e))
    })?;
    match db_access.by_slug(&project.slug)? {
        Some(p) => Err(warp::reject::custom(ProjectError::AlreadyExists(p.id))),
        None => match DBProject::create(&db_access, &project) {
            Ok(project) => {
                info!("project slug '{}' created", project.slug);
                Ok(with_status(json(&project), StatusCode::CREATED))
            }
            Err(error) => {
                error!("error creating the project '{:?}': {}", project, error);
                Err(warp::reject::custom(ProjectError::CannotCreate(
                    "error creating the project".to_string(),
                )))
            }
        },
    }
}

pub async fn update_handler(
    id: i32,
    user: GitHubUser,
    form: UpdateProject,
    db_access: impl DBProject + DBRole,
) -> Result<impl Reply, Rejection> {
    let user_roles = DBRole::user_roles(&db_access, &user.username)?;
    user_has_at_least_one_role(
        user_roles.clone(),
        vec![
            KudosRole::Admin,
        ],
    )?;
    match DBProject::by_id(&db_access,id)? {
        Some(p) => Ok(with_status(
            json(&DBProject::update(&db_access, p.id, &form)?),
            StatusCode::OK,
        )),
        None => Err(warp::reject::custom(ProjectError::NotFound(id))),
    }
}

pub async fn delete_handler(
    id: i32, 
    user: GitHubUser,
    db_access: impl DBProject + DBRole,
) -> Result<impl Reply, Rejection> {
    let user_roles = DBRole::user_roles(&db_access, &user.username)?;
    user_has_at_least_one_role(
        user_roles.clone(),
        vec![
            KudosRole::Admin,
        ],
    )?;
    match DBProject::by_id(&db_access, id)? {
        Some(p) => Ok(with_status(
            json(&DBProject::delete(&db_access, p.id)?),
            StatusCode::NO_CONTENT,
        )),
        None => Err(warp::reject::custom(ProjectError::NotFound(id))),
    }
}
pub async fn by_id(
    id: i32, 
    db_access: impl DBProject,
) -> Result<impl Reply, Rejection> {
    match db_access.by_id(id)? {
        None => Err(warp::reject::custom(ProjectError::NotFound(id)))?,
        Some(project) => Ok(json(&project)),
    }
}