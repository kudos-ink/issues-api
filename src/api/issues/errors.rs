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
}

impl fmt::Display for IssueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IssueError::AlreadyExists(id) => {
                write!(f, "Issue #{id} already exists")
            }
            IssueError::NotFound(id) => {
                write!(f, "Issue #{id} not found")
            }
            IssueError::RepositoryNotFound(id) => {
                write!(f, "Repository #{id} not found")
            }
        }
    }
}

impl Reject for IssueError {}

impl Reply for IssueError {
    fn into_response(self) -> Response {
        let code = match self {
            IssueError::AlreadyExists(_) => StatusCode::BAD_REQUEST,
            IssueError::NotFound(_) => StatusCode::NOT_FOUND,
            IssueError::RepositoryNotFound(_) => StatusCode::UNPROCESSABLE_ENTITY,
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
