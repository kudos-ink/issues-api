use std::fmt;

use serde_derive::Deserialize;
use thiserror::Error;
use warp::{
    http::StatusCode,
    reject::Reject,
    reply::{Reply, Response},
};

use crate::error_handler::ErrorResponse;

#[derive(Clone, Error, Debug, Deserialize, PartialEq)]
pub enum IssueError {
    IssueExists(i32),
    IssueNotFound(i32),
    IssueInvalidURL,
}

impl fmt::Display for IssueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IssueError::IssueExists(id) => {
                write!(f, "Issue #{} already exists", id)
            }
            IssueError::IssueNotFound(id) => {
                write!(f, "Issue #{} not found", id)
            }
            IssueError::IssueInvalidURL => {
                write!(f, "Issue url is invalid")
            }
        }
    }
}

impl Reject for IssueError {}

impl Reply for IssueError {
    fn into_response(self) -> Response {
        let code = match self {
            IssueError::IssueExists(_) => StatusCode::BAD_REQUEST,
            IssueError::IssueNotFound(_) => StatusCode::NOT_FOUND,
            IssueError::IssueInvalidURL => StatusCode::BAD_REQUEST,
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
