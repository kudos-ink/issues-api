use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, Reply},
};

use crate::{
    middlewares::github::model::GitHubUser
};

use super::{
    db::DBNotification,
    models::{DeleteNotification},
};


pub async fn by_github_id(
     user: GitHubUser,
     db_access: impl DBNotification) -> Result<impl Reply, Rejection> {
    let notifications = db_access.by_github_id(user.id, false)?;
    Ok(json(&notifications))
}


pub async fn delete_handler(
    id: i32,
    user: GitHubUser,
    db_access: impl DBNotification ,
) -> Result<impl Reply, Rejection> {
    let notification = DeleteNotification {
        id,
        github_id: Some(user.id),
    };
    db_access.delete(&notification)?;
    Ok(StatusCode::NO_CONTENT)

}

pub async fn delete_all_handler(
    user: GitHubUser,
    db_access: impl DBNotification ,
) -> Result<impl Reply, Rejection> {
    db_access.delete_all(user.id)?;
    Ok(StatusCode::NO_CONTENT)
}
