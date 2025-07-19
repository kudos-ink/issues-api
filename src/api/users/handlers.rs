use bytes::Buf;
use warp::{
    http::StatusCode,
    reject,
    reject::Rejection,
    reply::{json, with_status, Reply},
};

use crate::{api::{roles::{db::DBRole, models::KudosRole, utils::user_has_at_least_one_role}, users::models::UpdateEmailNotificationsUser}, middlewares::github::model::GitHubUser, types::PaginationParams};
use log::{error, info, warn};

use super::{
    db::DBUser,
    errors::UserError,
    models::{NewUser, QueryParams, UpdateUser},
};

pub async fn by_id(id: i32, db_access: impl DBUser) -> Result<impl Reply, Rejection> {
    match db_access.by_id(id)? {
        None => Err(warp::reject::custom(UserError::NotFound(id)))?,
        Some(user) => Ok(json(&user)),
    }
}

pub async fn by_github(user: GitHubUser, db_access: impl DBUser) -> Result<impl Reply, Rejection> {
    info!("get github user {:?}", user);
    by_username(user.username, db_access).await
}
pub async fn create_by_github(user: GitHubUser, db_access: impl DBUser) -> Result<impl Reply, Rejection> {
    info!("create github user {:?}", user);
    create_user( db_access, NewUser { username: user.username, avatar: Some(user.avatar_url), email: user.email, github_id: Some(user.id) })
}

pub async fn by_username(username: String, db_access: impl DBUser) -> Result<impl Reply, Rejection> {
    match db_access.by_username(&username)? {
        None => Err(warp::reject::custom(UserError::NotFoundByName(username)))?,
        Some(user) => Ok(json(&user)),
    }
}

pub async fn all_handler(
    db_access: impl DBUser,
    params: QueryParams,
    pagination: PaginationParams,
) -> Result<impl Reply, Rejection> {
    let users = db_access.all(params, pagination)?;
    Ok(json::<Vec<_>>(&users))
}

pub async fn create_handler(
    user: GitHubUser,
    buf: impl Buf,
    db_access: impl DBUser + DBRole,
) -> Result<impl Reply, Rejection> {
    let user_roles = DBRole::user_roles(&db_access, &user.username)?;
    user_has_at_least_one_role(
        user_roles.clone(),
        vec![
            KudosRole::Admin,
        ],
    )?;
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let user: NewUser = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid user '{e}'",);
        reject::custom(UserError::InvalidPayload(e))
    })?;

    create_user(db_access, user)
}

fn create_user(db_access: impl DBUser, user: NewUser) -> Result<warp::reply::WithStatus<warp::reply::Json>, Rejection> {
    match db_access.by_username(&user.username)? {
        Some(user) => {
            warn!("user already exists '{:?}'", user);
            Err(warp::reject::custom(UserError::AlreadyExists(user.id)))
        }
        None => match db_access.create(&user) {
            Ok(user) => {
                info!("user '{}' created", user.username);
                Ok(with_status(json(&user), StatusCode::CREATED))
            }
            Err(error) => {
                error!("error creating the user '{:?}': {}", user, error);
                Err(warp::reject::custom(UserError::CannotCreate(
                    "error creating the user".to_string(),
                )))
            }
        },
    }
}

pub async fn update_handler(
    id: i32,
    user: GitHubUser,
    buf: impl Buf,
    db_access: impl DBUser + DBRole,
) -> Result<impl Reply, Rejection> {
    let user_roles = DBRole::user_roles(&db_access, &user.username)?;
    user_has_at_least_one_role(
        user_roles.clone(),
        vec![
            KudosRole::Admin,
        ],
    )?;
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let user: UpdateUser = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid user '{e}'",);
        reject::custom(UserError::InvalidPayload(e))
    })?;
    update_user(db_access, id, user)
}

fn update_user(db_access: impl DBUser, id: i32, user: UpdateUser) -> Result<warp::reply::WithStatus<warp::reply::Json>, Rejection> {
    match db_access.by_id(id)? {
        Some(p) => Ok(with_status(
            json(&db_access.update(p.id, &user)?),
            StatusCode::OK,
        )),
        None => Err(warp::reject::custom(UserError::NotFound(id))),
    }
}
pub async fn update_user_github(user: GitHubUser, buf: impl Buf, db_access: impl DBUser) -> Result<warp::reply::WithStatus<warp::reply::Json>, Rejection> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let new_values: UpdateEmailNotificationsUser = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid user '{e}'",);
        reject::custom(UserError::InvalidPayload(e))
    })?;
    match db_access.by_github_id(user.id)? {
        Some(db_user) => Ok(with_status(
            json(&db_access.update(db_user.id, 
                &UpdateUser{ 
                    username: Some(db_user.username), 
                    avatar: db_user.avatar, 
                    github_id: Some(user.id), 
                    email_notifications_enabled: new_values.email_notifications_enabled 
                })?),
            StatusCode::OK,
        )),
        None => Err(warp::reject::custom(UserError::GithubNotFound(user.id))),
    }
}
pub async fn delete_handler(
    id: i32, 
    user: GitHubUser,
    db_access: impl DBUser + DBRole,
) -> Result<impl Reply, Rejection> {
    let user_roles = DBRole::user_roles(&db_access, &user.username)?;
    user_has_at_least_one_role(
        user_roles.clone(),
        vec![
            KudosRole::Admin,
        ],
    )?;
    match DBUser::by_id(&db_access, id)? {
        Some(p) => Ok(with_status(
            json(&DBUser::delete(&db_access, p.id)?),
            StatusCode::NO_CONTENT,
        )),
        None => Err(warp::reject::custom(UserError::NotFound(id))),
    }
}
