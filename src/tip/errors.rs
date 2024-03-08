use std::fmt;

use serde_derive::Deserialize;
use thiserror::Error;
use warp::{
    http::StatusCode,
    reject::Reject,
    reply::{Reply, Response},
};

use crate::{handlers::ErrorResponse, types::TipId};

#[derive(Clone, Error, Debug, Deserialize)]
pub enum TipError {
    TipExists(TipId),
    InvalidUpdateRequest(TipId),
    NotFound,
}

impl fmt::Display for TipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TipError::TipExists(id) => {
                write!(f, "Tip #{:?} already exists", id)
            },
            TipError::NotFound => {
                write!(f, "Tip not found")
            },
            TipError::InvalidUpdateRequest(id) => {
                write!(f, "Invalid body for Tip #{:?} update", id)
            }
        }
    }
}

impl Reject for TipError {}

impl Reply for TipError {
    fn into_response(self) -> Response {
        let code = match self {
            TipError::TipExists(_) => StatusCode::BAD_REQUEST,
            TipError::InvalidUpdateRequest(_) => StatusCode::BAD_REQUEST,
            TipError::NotFound => StatusCode::NOT_FOUND,
        };
        let message = self.to_string();
        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
