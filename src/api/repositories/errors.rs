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
pub enum RepositoryError {
    AlreadyExists(i32),
    NotFound(i32),
    NotFoundByName(String),
    ProjectNotFound(i32),
    InvalidPayload(String),
    CannotCreate(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::AlreadyExists(id) => {
                write!(f, "Repository '{id}' already exists")
            }
            RepositoryError::NotFound(id) => {
                write!(f, "Repository '{id}' not found")
            }
            RepositoryError::NotFoundByName(name) => {
                write!(f, "Repository '{name}' not found")
            }
            RepositoryError::InvalidPayload(error) => {
                write!(f, "Invalid payload: {error}")
            },
            RepositoryError::CannotCreate(err) => {
                write!(f, "error creating the repository: {err}")
            },
            RepositoryError::ProjectNotFound(id) => {
                write!(f, "Project id '{id}' does not exist")},
        }
    }
}

impl Reject for RepositoryError {}

impl Reply for RepositoryError {
    fn into_response(self) -> Response {
        let code = match self {
            RepositoryError::AlreadyExists(_) => StatusCode::BAD_REQUEST,
            RepositoryError::NotFound(_) => StatusCode::NOT_FOUND,
            RepositoryError::NotFoundByName(_) => StatusCode::NOT_FOUND,
            RepositoryError::InvalidPayload(_) => StatusCode::UNPROCESSABLE_ENTITY,
            RepositoryError::CannotCreate(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RepositoryError::ProjectNotFound(_) => StatusCode::UNPROCESSABLE_ENTITY,
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
