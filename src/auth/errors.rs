use serde::Deserialize;
use std::fmt;
use thiserror::Error;
use warp::{http::StatusCode, reject::Reject, reply::Response, Reply};

use crate::errors::ErrorResponse;

#[derive(Clone, Error, Debug, Deserialize, PartialEq)]
pub enum AuthenticationError {
    WrongCredentials,
    BasicToken,
    NoAuthHeader,
    InvalidAuthHeader,
}

impl fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthenticationError::WrongCredentials => {
                write!(f, "Wrong credentials")
            }
            AuthenticationError::BasicToken => {
                write!(f, "Basic Token Error")
            }
            AuthenticationError::NoAuthHeader => write!(f, "No Authorization Header"),
            AuthenticationError::InvalidAuthHeader => {
                write!(f, "Invalid Authorization Header")
            }
        }
    }
}

impl Reject for AuthenticationError {}

impl Reply for AuthenticationError {
    fn into_response(self) -> Response {
        let code = StatusCode::UNAUTHORIZED;
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
