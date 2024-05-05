use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, with_status, Reply},
};

use crate::types::PaginationParams;

use super::{
    db::DBUser,
    errors::UserError,
    models::{NewUser, UpdateUser},
};

pub async fn by_id(id: i32, db_access: impl DBUser) -> Result<impl Reply, Rejection> {
    match db_access.by_id(id)? {
        None => Err(warp::reject::custom(UserError::NotFound(id)))?,
        Some(user) => Ok(json(&user)),
    }
}

pub async fn all_handler(
    db_access: impl DBUser,
    pagination: PaginationParams,
) -> Result<impl Reply, Rejection> {
    let users = db_access.all(pagination)?;
    Ok(json::<Vec<_>>(&users))
}

pub async fn create_handler(
    user: NewUser,
    db_access: impl DBUser,
) -> Result<impl Reply, Rejection> {
    match db_access.by_username(&user.username)? {
        Some(r) => Err(warp::reject::custom(UserError::AlreadyExists(r.id))),
        None => Ok(with_status(
            json(&db_access.create(&user)?),
            StatusCode::CREATED,
        )),
    }
}
pub async fn update_handler(
    id: i32,
    repo: UpdateUser,
    db_access: impl DBUser,
) -> Result<impl Reply, Rejection> {
    match db_access.by_id(id)? {
        Some(p) => Ok(with_status(
            json(&db_access.update(p.id, &repo)?),
            StatusCode::OK,
        )),
        None => Err(warp::reject::custom(UserError::NotFound(id))),
    }
}
pub async fn delete_handler(id: i32, db_access: impl DBUser) -> Result<impl Reply, Rejection> {
    match db_access.by_id(id)? {
        Some(p) => Ok(with_status(json(&db_access.delete(p.id)?), StatusCode::OK)),
        None => Err(warp::reject::custom(UserError::NotFound(id))),
    }
}
