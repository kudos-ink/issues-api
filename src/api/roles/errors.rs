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
pub enum RoleError {
    AlreadyExists(i32),
    NotFound(i32),
    UserNotFound(i32),
    ProjectNotFound(i32),
    RoleNotFound(i32),
    InvalidPayload(String),
    AssignationAlreadyExists(),
    CannotCreate(String),
    CannotUpdate(String),
    CannotDelete(String),
    MissingRole(String),
}

impl fmt::Display for RoleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoleError::AlreadyExists(id) => write!(f, "Role #{id} already exists"),
            RoleError::AssignationAlreadyExists() => write!(f, "Assignation already exists"),
            RoleError::NotFound(id) => write!(f, "Role #{id} not found"),
            RoleError::UserNotFound(id) => write!(f, "User #{id} not found"),
            RoleError::ProjectNotFound(id) => write!(f, "Project #{id} not found"),
            RoleError::RoleNotFound(id) => write!(f, "Role #{id} not found"),
            RoleError::InvalidPayload(error) => write!(f, "Invalid payload: {error}"),
            RoleError::CannotCreate(error) => write!(f, "Error creating the role: {error}"),
            RoleError::CannotUpdate(error) => write!(f, "Error updating the role: {error}"),
            RoleError::CannotDelete(error) => write!(f, "Error deleting the role: {error}"),
            RoleError::MissingRole(error) => write!(f, "Roles needed for the action: {error}"),
        }
    }
}

impl Reject for RoleError {}

impl Reply for RoleError {
    fn into_response(self) -> Response {
        let code = match self {
            RoleError::AlreadyExists(_) => StatusCode::BAD_REQUEST,
            RoleError::NotFound(_) => StatusCode::NOT_FOUND,
            RoleError::UserNotFound(_) => StatusCode::NOT_FOUND,
            RoleError::ProjectNotFound(_) => StatusCode::NOT_FOUND,
            RoleError::RoleNotFound(_) => StatusCode::NOT_FOUND,
            RoleError::InvalidPayload(_) => StatusCode::NOT_FOUND,
            RoleError::CannotCreate(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RoleError::CannotUpdate(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RoleError::CannotDelete(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RoleError::AssignationAlreadyExists() => StatusCode::BAD_REQUEST,
            RoleError::MissingRole(_) => StatusCode::FORBIDDEN,
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
