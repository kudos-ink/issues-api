use bytes::Buf;
use log::{error, info, warn};
use warp::{http::StatusCode, reject, reply::{json, with_status}, Rejection, Reply};
use crate::api::users::db::DBUser;
use crate::api::users::errors::UserError;
use crate::middlewares::github::model::GitHubUser;
use super::db::DBComment;
use super::models::{NewComment, CreateCommentPayload};
use super::errors::CommentError;

pub async fn get_comments_handler(id: i32, db_access: impl DBComment) -> Result<impl Reply, Rejection> {
    let comments = db_access.by_task_id(id)?;
    Ok(json(&comments))
}

pub async fn create_comment_handler(
    task_id: i32,
    user: GitHubUser,
    buf: impl Buf,
    db_access: impl DBComment + DBUser,
) -> Result<impl Reply, Rejection> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let mut comment_payload: CreateCommentPayload = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid comment '{e}'",);
        reject::custom(CommentError::InvalidPayload(e))
    })?;


    let db_user = db_access.by_github_id(user.id)?
        .ok_or_else(|| reject::custom(UserError::GithubNotFound(user.id)))?;

    let new_comment = NewComment {
        content: comment_payload.content,
        parent_comment_id: comment_payload.parent_comment_id,
        task_id,
        user_id: db_user.id
    };


    let saved = DBComment::create(&db_access, &new_comment)?;
    Ok(with_status(json(&saved), StatusCode::CREATED))
}