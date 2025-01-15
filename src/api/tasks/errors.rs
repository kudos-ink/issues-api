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
pub enum TaskError {
    AlreadyExists(i32),
    NotFound(i32),
    UserNotFound(i32),
    UserCannotVote(),
    ProjectNotFound(i32),
    RepositoryNotFound(i32),
    InvalidTask(String),
    InvalidPayload(String),
    UserAlreadyVoted(),
    CannotCreate(String),
    CannotUpdate(String),
    CannotDelete(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::AlreadyExists(id) => write!(f, "Task #{id} already exists"),
            TaskError::NotFound(id) => write!(f, "Task #{id} not found"),
            TaskError::UserNotFound(id) => write!(f, "User #{id} not found"),
            TaskError::ProjectNotFound(id) => write!(f, "Project #{id} not found"),
            TaskError::InvalidPayload(error) => write!(f, "Invalid payload: {error}"),
            TaskError::CannotCreate(error) => write!(f, "Error creating the task: {error}"),
            TaskError::CannotUpdate(error) => write!(f, "Error updating the task: {error}"),
            TaskError::CannotDelete(error) => write!(f, "Error deleting the task: {error}"),
            TaskError::RepositoryNotFound(id) => write!(f, "Repository #{id} not found"),
            TaskError::InvalidTask(error) => write!(f, "Invalid task: {error}"),
            TaskError::UserAlreadyVoted() => write!(f, "User already voted"),
            TaskError::UserCannotVote() => write!(f, "User cannot vote"),
        }
    }
}

impl Reject for TaskError {}

impl Reply for TaskError {
    fn into_response(self) -> Response {
        let code = match self {
            TaskError::AlreadyExists(_) => StatusCode::BAD_REQUEST,
            TaskError::NotFound(_) => StatusCode::NOT_FOUND,
            TaskError::UserNotFound(_) => StatusCode::NOT_FOUND,
            TaskError::ProjectNotFound(_) => StatusCode::NOT_FOUND,
            TaskError::InvalidPayload(_) => StatusCode::NOT_FOUND,
            TaskError::CannotCreate(_) => StatusCode::INTERNAL_SERVER_ERROR,
            TaskError::CannotUpdate(_) => StatusCode::INTERNAL_SERVER_ERROR,
            TaskError::CannotDelete(_) => StatusCode::INTERNAL_SERVER_ERROR,
            TaskError::RepositoryNotFound(_) => StatusCode::NOT_FOUND,
            TaskError::InvalidTask(_) => StatusCode::BAD_REQUEST,
            TaskError::UserAlreadyVoted() => StatusCode::BAD_REQUEST,
            TaskError::UserCannotVote() => StatusCode::FORBIDDEN,
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
