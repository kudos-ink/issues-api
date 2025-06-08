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
pub enum UserSubscriptionError {
    AlreadyExists(i32),
    NotFound(i32),
    NotFoundByGithubUser(i64),
    CannotCreate(String),
    InvalidPayload(String),
    CannotDelete(String),
}

impl fmt::Display for UserSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserSubscriptionError::AlreadyExists(id) => write!(f, "Subscription #{id} already exists"),
            UserSubscriptionError::NotFound(id) => write!(f, "Subscription #{id} not found"),
            UserSubscriptionError::NotFoundByGithubUser(github_id) => write!(f, "No subscriptions found for github user #{github_id}"),
            UserSubscriptionError::CannotCreate(error) => write!(f, "Subscription cannot be created: {error}"),
            UserSubscriptionError::CannotDelete(error) => write!(f, "Subscription cannot be deleted: {error}"),
            UserSubscriptionError::InvalidPayload(error) => write!(f, "Cannot create the subscription: {error}"),
        }
    }
}

impl Reject for UserSubscriptionError {}

impl Reply for UserSubscriptionError {
    fn into_response(self) -> Response {
        let code = match self {
            UserSubscriptionError::AlreadyExists(_) => StatusCode::BAD_REQUEST,
            UserSubscriptionError::NotFound(_) => StatusCode::NOT_FOUND,
            UserSubscriptionError::NotFoundByGithubUser(_) => StatusCode::NOT_FOUND,
            UserSubscriptionError::CannotCreate(_) => StatusCode::UNPROCESSABLE_ENTITY,
            UserSubscriptionError::CannotDelete(_) => StatusCode::UNPROCESSABLE_ENTITY,
            UserSubscriptionError::InvalidPayload(_) => StatusCode::UNPROCESSABLE_ENTITY,
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
