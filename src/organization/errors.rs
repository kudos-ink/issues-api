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
pub enum OrganizationError {
    OrganizationExists(i32),
    OrganizationNotFound(i32),
}

impl fmt::Display for OrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrganizationError::OrganizationExists(id) => {
                write!(f, "Organization #{} already exists", id)
            }
            OrganizationError::OrganizationNotFound(id) => {
                write!(f, "Organization #{} not found", id)
            }
        }
    }
}

impl Reject for OrganizationError {}

impl Reply for OrganizationError {
    fn into_response(self) -> Response {
        let code = match self {
            OrganizationError::OrganizationExists(_) => StatusCode::BAD_REQUEST,
            OrganizationError::OrganizationNotFound(_) => StatusCode::NOT_FOUND,
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
