use std::fmt;

use serde_derive::Deserialize;
use thiserror::Error;
use warp::{
    http::StatusCode,
    reject::Reject,
    reply::{Reply, Response},
};

use crate::errors::ErrorResponse;

#[derive(Clone, Error, Debug, Deserialize, PartialEq)]
pub enum CommentError {
    NotFound(i32),
    TaskNotFound(i32),
    UserNotFound(i32),
    InvalidPayload(String),
    CannotCreate(String),
    CannotDelete(String),
    UnauthorizedAction,
}

impl fmt::Display for CommentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommentError::NotFound(id) => write!(f, "Comment #{id} not found"),
            CommentError::TaskNotFound(id) => write!(f, "Task #{id} not found, cannot create comment"),
            CommentError::UserNotFound(id) => write!(f, "User #{id} not found"),
            CommentError::InvalidPayload(error) => write!(f, "Invalid payload: {error}"),
            CommentError::CannotCreate(error) => write!(f, "Error creating the comment: {error}"),
            CommentError::CannotDelete(error) => write!(f, "Error deleting the comment: {error}"),
            CommentError::UnauthorizedAction => write!(f, "User is not authorized to perform this action"),
        }
    }
}

impl Reject for CommentError {}

impl Reply for CommentError {
    fn into_response(self) -> Response {
        let code = match self {
            CommentError::NotFound(_) => StatusCode::NOT_FOUND,
            CommentError::TaskNotFound(_) => StatusCode::NOT_FOUND,
            CommentError::UserNotFound(_) => StatusCode::NOT_FOUND,
            CommentError::InvalidPayload(_) => StatusCode::UNPROCESSABLE_ENTITY,
            CommentError::CannotCreate(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CommentError::CannotDelete(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CommentError::UnauthorizedAction => StatusCode::FORBIDDEN,
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}