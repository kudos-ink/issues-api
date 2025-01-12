


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
pub enum TeamError {
    NotFound(i32),
    MemberNotFound(i32),
    InvalidPayload(String),
    CannotCreate(String),
    CannotUpdate(String),
    CannotDelete(String),
}

impl fmt::Display for TeamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TeamError::NotFound(id) => write!(f, "Team #{id} not found"),
            TeamError::MemberNotFound(id) => write!(f, "Team member #{id} not found"),
            TeamError::InvalidPayload(error) => write!(f, "Invalid payload: {error}"),
            TeamError::CannotCreate(error) => write!(f, "Error creating the resource: {error}"),
            TeamError::CannotUpdate(error) => write!(f, "Error updating the resource: {error}"),
            TeamError::CannotDelete(error) => write!(f, "Error deleting the resource: {error}"),
        }
    }
}

impl Reject for TeamError {}

impl Reply for TeamError {
    fn into_response(self) -> Response {
        let code = match self {
            TeamError::NotFound(_) | TeamError::MemberNotFound(_) => StatusCode::NOT_FOUND,
            TeamError::InvalidPayload(_) => StatusCode::UNPROCESSABLE_ENTITY,
            TeamError::CannotCreate(_) | TeamError::CannotUpdate(_) | TeamError::CannotDelete(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
