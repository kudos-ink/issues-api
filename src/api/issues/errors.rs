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
pub enum IssueError {
    AlreadyExists(i32),
    NotFound(i32),
    RepositoryNotFound(i32),
    InvalidPayload(String),
    CannotCreate(String),
    CannotUpdate(String),
}

impl fmt::Display for IssueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IssueError::AlreadyExists(id) => write!(f, "Issue #{id} already exists"),
            IssueError::NotFound(id) => write!(f, "Issue #{id} not found"),
            IssueError::InvalidPayload(error) => write!(f, "Invalid payload: {error}"),
            IssueError::RepositoryNotFound(id) => write!(f, "Repository #{id} not found"),
            IssueError::CannotCreate(error) => write!(f, "error creating the issue: {error}"),
            IssueError::CannotUpdate(error) => write!(f, "error updating the issue: {error}"),
        }
    }
}

impl Reject for IssueError {}

impl Reply for IssueError {
    fn into_response(self) -> Response {
        let code = match self {
            IssueError::AlreadyExists(_) => StatusCode::BAD_REQUEST,
            IssueError::NotFound(_) => StatusCode::NOT_FOUND,
            IssueError::InvalidPayload(_) => StatusCode::UNPROCESSABLE_ENTITY,
            IssueError::CannotCreate(_) => StatusCode::INTERNAL_SERVER_ERROR,
            IssueError::CannotUpdate(_) => StatusCode::INTERNAL_SERVER_ERROR,
            IssueError::RepositoryNotFound(_) => StatusCode::UNPROCESSABLE_ENTITY,
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
