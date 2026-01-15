use bytes::Buf;
use log::warn;
use warp::{http::StatusCode, reject, reply::{json, with_status}, Rejection, Reply};
use crate::api::users::db::DBUser;
use crate::api::users::errors::UserError;
use crate::middlewares::github::model::GitHubUser;
use super::db::DBComment;
use super::models::{NewComment, CreateCommentPayload};
use super::errors::CommentError;
use crate::api::roles::{db::DBRole, models::KudosRole, utils::user_has_at_least_one_role};

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
    let comment_payload: CreateCommentPayload = serde_path_to_error::deserialize(des).map_err(|e| {
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

pub async fn by_comment_id_handler(
    comment_id: i32,
    db_access: impl DBComment,
) -> Result<impl Reply, Rejection> {
    match db_access.by_comment_id(comment_id)? {
        Some(comment) => Ok(json(&comment)),
        None => Err(reject::custom(CommentError::NotFound(comment_id))),
    }
}

pub async fn delete_comment_handler(
    comment_id: i32,
    user: GitHubUser,
    db_access: impl DBComment + DBUser + DBRole,
) -> Result<impl Reply, Rejection> {
    let db_user = db_access.by_github_id(user.id)?
        .ok_or_else(|| reject::custom(UserError::GithubNotFound(user.id)))?;

    let comment = db_access.by_comment_id(comment_id)?
        .ok_or_else(|| reject::custom(CommentError::NotFound(comment_id)))?;

    let user_roles = db_access.user_roles(&user.username)?;
    let is_admin = user_has_at_least_one_role(user_roles, vec![KudosRole::Admin]).is_ok();
    
    if comment.user.id != db_user.id && !is_admin {
        return Err(reject::custom(CommentError::UnauthorizedAction));
    }

    if db_access.has_replies(comment_id)? {
        db_access.soft_delete(comment_id)?;
    } else {
        db_access.hard_delete(comment_id)?;
    }

    Ok(StatusCode::NO_CONTENT)
}