use serde::Serialize;
use serde_derive::Deserialize;
use std::convert::Infallible;
use warp::{hyper::StatusCode, Rejection, Reply};

use crate::{
    api::{
        issues::errors::IssueError, 
        projects::errors::ProjectError, 
        repositories::errors::RepositoryError, 
        roles::errors::RoleError, 
        tasks::errors::TaskError, 
        users::errors::UserError,
        subscriptions::errors::UserSubscriptionError,
        notifications::errors::NotificationError,
        comments::errors::CommentError
    },
    db::errors::DBError,
    middlewares::errors::AuthenticationError,
};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ErrorResponse {
    pub message: String,
}

pub async fn error_handler(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    if let Some(e) = err.find::<IssueError>() {
        return Ok(e.clone().into_response());
    } else if let Some(e) = err.find::<RepositoryError>() {
        return Ok(e.clone().into_response());
    } else if let Some(e) = err.find::<ProjectError>() {
        return Ok(e.clone().into_response());
    } else if let Some(e) = err.find::<UserError>() {
        return Ok(e.clone().into_response());
    } else if let Some(e) = err.find::<RoleError>() {
        return Ok(e.clone().into_response());
    } else if let Some(e) = err.find::<TaskError>() {
        return Ok(e.clone().into_response());
    } else if let Some(e) = err.find::<UserSubscriptionError>() {
        return Ok(e.clone().into_response());
    } else if let Some(e) = err.find::<NotificationError>() {
        return Ok(e.clone().into_response());
    } else if let Some(e) = err.find::<CommentError>() {
        return Ok(e.clone().into_response());
    }
    // TODO: add more errors

    let (status, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Resource not found".to_string())
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        eprintln!("BodyDeserializeError error: {:?}", e);
        (StatusCode::BAD_REQUEST, "Invalid request body".to_string())
    } else if let Some(e) = err.find::<warp::reject::InvalidQuery>() {
        eprintln!("InvalidQuery error: {:?}", e);
        (
            StatusCode::BAD_REQUEST,
            "Invalid query parameters".to_string(),
        )
    } else if let Some(e) = err.find::<AuthenticationError>() {
        eprintln!("AuthenticationError: {e}");
        (
            StatusCode::UNAUTHORIZED,
            format!("AuthenticationError - {e}"),
        )
    } else if let Some(db_error) = err.find::<DBError>() {
        match db_error {
            DBError::DBPoolConnection(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database connection error".to_string(),
            ),
            DBError::DBQuery(_) => (StatusCode::BAD_REQUEST, "Database query failed".to_string()),
            DBError::ReadFile(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "File read error".to_string(),
            ),
            DBError::DBTimeout(_) => (
                StatusCode::REQUEST_TIMEOUT,
                "Database operation timed out".to_string(),
            ),
        }
    } else {
        eprintln!("Unhandled error: {:?}", err); // Ensure all unexpected errors are logged.
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    };

    let json = warp::reply::json(&ErrorResponse { message });
    let response = warp::reply::with_status(json, status).into_response();
    Ok(response)
}
