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
pub enum ContributionError {
    ContributionExists(i64),
    ContributionNotFound(i64),
}

impl fmt::Display for ContributionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContributionError::ContributionExists(id) => {
                write!(f, "Contribution #{} already exists", id)
            }
            ContributionError::ContributionNotFound(id) => {
                write!(f, "Contribution #{} not found", id)
            }
        }
    }
}

impl Reject for ContributionError {}

impl Reply for ContributionError {
    fn into_response(self) -> Response {
        let code = match self {
            ContributionError::ContributionExists(_) => StatusCode::BAD_REQUEST,
            ContributionError::ContributionNotFound(_) => StatusCode::NOT_FOUND,
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse {
            message: message.into(),
        });

        warp::reply::with_status(json, code).into_response()
    }
}
