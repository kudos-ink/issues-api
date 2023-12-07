use serde::Serialize;
use serde_derive::Deserialize;
use std::convert::Infallible;
use warp::{hyper::StatusCode, Rejection, Reply};

use crate::{contributions::errors::ContributionError, db::errors::DBError};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ErrorResponse {
    pub message: String,
}

pub async fn error_handler(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    if let Some(e) = err.find::<ContributionError>() {
        Ok(e.clone().into_response())
    } else if let Some(e) = err.find::<DBError>() {
        let (code, message) = match e {
            DBError::DBPoolConnection(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database connection error",
            ),
            DBError::DBQuery(_) => (StatusCode::BAD_REQUEST, "Database query failed"),
            DBError::DBInit(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error initializing database",
            ),
            DBError::ReadFile(_) => (StatusCode::INTERNAL_SERVER_ERROR, "File read error"),
            DBError::DBTimeout(_) => (StatusCode::REQUEST_TIMEOUT, "Database operation timed out"),
        };

        let json = warp::reply::json(&ErrorResponse {
            message: message.to_string(),
        });

        Ok(warp::reply::with_status(json, code).into_response())
    } else {
        let code;
        let message;

        if err.is_not_found() {
            // Handle not found errors
            code = StatusCode::NOT_FOUND;
            message = "Not Found";
        } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
            // Handle invalid body errors
            code = StatusCode::BAD_REQUEST;
            message = "Invalid Body";
        } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
            // Handle method not allowed errors
            code = StatusCode::METHOD_NOT_ALLOWED;
            message = "Method Not Allowed";
        } else {
            // Handle all other errors
            eprintln!("Unhandled error: {:?}", err);
            code = StatusCode::INTERNAL_SERVER_ERROR;
            message = "Internal Server Error";
        }

        let json = warp::reply::json(&ErrorResponse {
            message: message.into(),
        });

        Ok(warp::reply::with_status(json, code).into_response())
    }
}
