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
pub enum UserError {
    AlreadyExists(i32),
    NotFound(i32),
    NotFoundByName(String),
    CannotCreate(String),
    CannotUpdate(i32, String),
    InvalidPayload(String),
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::AlreadyExists(id) => write!(f, "User #{id} already exists"),
            UserError::NotFound(id) => write!(f, "User #{id} not found"),
            UserError::NotFoundByName(name) => write!(f, "User {name} not found"),
            UserError::CannotCreate(error) => write!(f, "User cannot be created: {error}"),
            UserError::CannotUpdate(id, error) => {
                write!(f, "User #{id} cannot be updated: {error}")
            }
            UserError::InvalidPayload(error) => write!(f, "Cannot create the user: {error}"),
        }
    }
}

impl Reject for UserError {}

impl Reply for UserError {
    fn into_response(self) -> Response {
        let code = match self {
            UserError::AlreadyExists(_) => StatusCode::BAD_REQUEST,
            UserError::NotFound(_) => StatusCode::NOT_FOUND,
            UserError::NotFoundByName(_) => StatusCode::NOT_FOUND,
            UserError::CannotCreate(_) => StatusCode::UNPROCESSABLE_ENTITY,
            UserError::CannotUpdate(_, _) => StatusCode::UNPROCESSABLE_ENTITY,
            UserError::InvalidPayload(_) => StatusCode::UNPROCESSABLE_ENTITY,
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
