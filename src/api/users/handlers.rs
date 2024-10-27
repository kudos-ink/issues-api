use bytes::Buf;
use warp::{
    http::StatusCode,
    reject,
    reject::Rejection,
    reply::{json, with_status, Reply},
};

use crate::types::PaginationParams;
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
    buf: impl Buf,
    db_access: impl DBUser,
) -> Result<impl Reply, Rejection> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let user: NewUser = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid user '{e}'",);
        reject::custom(UserError::InvalidPayload(e))
    })?;
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
    buf: impl Buf,
    db_access: impl DBUser,
) -> Result<impl Reply, Rejection> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let user: UpdateUser = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid user '{e}'",);
        reject::custom(UserError::InvalidPayload(e))
    })?;
    match db_access.by_id(id)? {
        Some(p) => Ok(with_status(
            json(&db_access.update(p.id, &user)?),
            StatusCode::OK,
        )),
        None => Err(warp::reject::custom(UserError::NotFound(id))),
    }
}
pub async fn delete_handler(id: i32, db_access: impl DBUser) -> Result<impl Reply, Rejection> {
    match db_access.by_id(id)? {
        Some(p) => Ok(with_status(
            json(&db_access.delete(p.id)?),
            StatusCode::NO_CONTENT,
        )),
        None => Err(warp::reject::custom(UserError::NotFound(id))),
    }
}
