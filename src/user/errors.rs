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
pub enum UserError {
    UserExists(i32),
    UserNotFound(i32),
    UserNotFoundByName(String),
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::UserExists(id) => {
                write!(f, "User #{} already exists", id)
            }
            UserError::UserNotFound(id) => {
                write!(f, "User #{} not found", id)
            }
            UserError::UserNotFoundByName(name) => {
                write!(f, "User {} not found", name)
            }
        }
    }
}

impl Reject for UserError {}

impl Reply for UserError {
    fn into_response(self) -> Response {
        let code = match self {
            UserError::UserExists(_) => StatusCode::BAD_REQUEST,
            UserError::UserNotFound(_) => StatusCode::NOT_FOUND,
            UserError::UserNotFoundByName(_) => StatusCode::NOT_FOUND,
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
