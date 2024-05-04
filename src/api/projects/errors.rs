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
pub enum ProjectError {
    AlreadyExists(i32),
    NotFound(i32),
    NotFoundBySlug(String),
}

impl fmt::Display for ProjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectError::AlreadyExists(id) => {
                write!(f, "Project #{id} already exists")
            }
            ProjectError::NotFound(id) => {
                write!(f, "Project #{id} not found")
            }
            ProjectError::NotFoundBySlug(slug) => {
                write!(f, "Project {slug} not found")
            }
        }
    }
}

impl Reject for ProjectError {}

impl Reply for ProjectError {
    fn into_response(self) -> Response {
        let code = match self {
            ProjectError::AlreadyExists(_) => StatusCode::BAD_REQUEST,
            ProjectError::NotFound(_) => StatusCode::NOT_FOUND,
            ProjectError::NotFoundBySlug(_) => StatusCode::NOT_FOUND,
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
