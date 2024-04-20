use std::fmt::{self};

use serde_derive::{Deserialize, Serialize};
use thiserror::Error;
use warp::{
    http::StatusCode,
    reject::Reject,
    reply::{Reply, Response},
};

use crate::handlers::ErrorResponse;

#[derive(Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    // pub maintainers: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub repositories: Option<Vec<i32>>,
}
#[derive(Serialize, Deserialize)]
pub struct PatchUser {
    pub repositories: Vec<i32>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
}

impl UserResponse {
    pub fn of(user: User) -> UserResponse {
        UserResponse {
            id: user.id,
            username: user.username,
        }
    }
}

#[derive(Default)]
pub struct UsersRelations {
    pub wishes: bool,
    pub tips: bool,
    pub maintainers: bool,
    pub issues: bool,
}
// query args

#[derive(Serialize, Deserialize, Default)]
pub struct GetUserQuery {
    pub wishes: Option<bool>,
    pub tips: Option<bool>,
    pub maintainers: Option<bool>,
    pub issues: Option<bool>,
    // TODO: add filters
    // pub is_maintainer: Option<bool>,
    // pub has_tips: Option<bool>,
    // pub has_issues: Option<bool>,
    // pub has_wishes: Option<bool>,
    // pub has_wishes: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct UserSort {
    pub field: String,
    pub order: String,
}
impl UserSort {
    pub fn new(field: &str, descending: bool) -> Result<Self, UserSortError> {
        if field != "id" && field != "username" {
            return Err(UserSortError::InvalidSortBy(field.to_owned()));
        }

        Ok(Self {
            field: format!("users.{field}"),
            order: {
                if descending {
                    "DESC".to_string()
                } else {
                    "ASC".to_string()
                }
            },
        })
    }
}

impl Default for UserSort {
    fn default() -> Self {
        UserSort {
            field: "id".to_string(),
            order: "ASC".to_string(),
        }
    }
}

#[derive(Clone, Error, Debug, Deserialize, PartialEq)]
pub enum UserSortError {
    InvalidSortBy(String),
}

impl fmt::Display for UserSortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserSortError::InvalidSortBy(field) => {
                write!(f, "Sort by {} is invalid", field)
            }
        }
    }
}

impl Reject for UserSortError {}

impl Reply for UserSortError {
    fn into_response(self) -> Response {
        let status_code = match self {
            UserSortError::InvalidSortBy(_) => StatusCode::BAD_REQUEST,
        };
        let code = status_code;
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
