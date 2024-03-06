use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, Reply},
};

use super::{
    db::DBUser,
    errors::UserError,
    models::{UserRequest, UserResponse},
};

pub async fn create_user_handler(
    body: UserRequest,
    db_access: impl DBUser,
) -> Result<impl Reply, Rejection> {
    match db_access.get_user_by_username(&body.username).await? {
        Some(u) => Err(warp::reject::custom(UserError::UserExists(u.id)))?,
        None => Ok(json(&UserResponse::of(db_access.create_user(body).await?))),
    }
}

pub async fn get_user_handler(id: i32, db_access: impl DBUser) -> Result<impl Reply, Rejection> {
    match db_access.get_user(id).await? {
        None => Err(warp::reject::custom(UserError::UserNotFound(id)))?,
        Some(user) => Ok(json(&UserResponse::of(user))),
    }
}

pub async fn get_users_handler(db_access: impl DBUser) -> Result<impl Reply, Rejection> {
    let users = db_access.get_users().await?;
    Ok(json::<Vec<_>>(
        &users.into_iter().map(UserResponse::of).collect(),
    ))
}

pub async fn delete_user_handler(id: i32, db_access: impl DBUser) -> Result<impl Reply, Rejection> {
    match db_access.get_user(id).await? {
        Some(_) => {
            let _ = &db_access.delete_user(id).await?;
            Ok(StatusCode::OK)
        }
        None => Err(warp::reject::custom(UserError::UserNotFound(id)))?,
    }
}
