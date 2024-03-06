use std::fmt;

use serde_derive::Deserialize;
use thiserror::Error;
use warp::{
    http::StatusCode,
    reject::Reject,
    reply::{Reply, Response},
};

use crate::handlers::ErrorResponse;

#[derive(Clone, Error, Debug, Deserialize, PartialEq)]
pub enum RepositoryError {
    RepositoryExists(i32),
    RepositoryNotFound(i32),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::RepositoryExists(id) => {
                write!(f, "Repository #{} already exists", id)
            }
            RepositoryError::RepositoryNotFound(id) => {
                write!(f, "Repository #{} not found", id)
            }
        }
    }
}

impl Reject for RepositoryError {}

impl Reply for RepositoryError {
    fn into_response(self) -> Response {
        let code = match self {
            RepositoryError::RepositoryExists(_) => StatusCode::BAD_REQUEST,
            RepositoryError::RepositoryNotFound(_) => StatusCode::NOT_FOUND,
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
