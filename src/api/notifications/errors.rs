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
pub enum NotificationError {
    AlreadyExists(i32),
    NotFound(i32),
    NotFoundByGithubUser(i64),
    CannotCreate(String),
    InvalidPayload(String),
    CannotDelete(String),
}

impl fmt::Display for NotificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotificationError::AlreadyExists(id) => write!(f, "Notification #{id} already exists"),
            NotificationError::NotFound(id) => write!(f, "Notification #{id} not found"),
            NotificationError::NotFoundByGithubUser(github_id) => write!(f, "No Notifications found for github user #{github_id}"),
            NotificationError::CannotCreate(error) => write!(f, "Notification cannot be created: {error}"),
            NotificationError::CannotDelete(error) => write!(f, "Notification cannot be deleted: {error}"),
            NotificationError::InvalidPayload(error) => write!(f, "Cannot create the Notification: {error}"),
        }
    }
}

impl Reject for NotificationError {}

impl Reply for NotificationError {
    fn into_response(self) -> Response {
        let code = match self {
            NotificationError::AlreadyExists(_) => StatusCode::BAD_REQUEST,
            NotificationError::NotFound(_) => StatusCode::NOT_FOUND,
            NotificationError::NotFoundByGithubUser(_) => StatusCode::NOT_FOUND,
            NotificationError::CannotCreate(_) => StatusCode::UNPROCESSABLE_ENTITY,
            NotificationError::CannotDelete(_) => StatusCode::UNPROCESSABLE_ENTITY,
            NotificationError::InvalidPayload(_) => StatusCode::UNPROCESSABLE_ENTITY,
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
