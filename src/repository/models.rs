use std::fmt;

use serde_derive::{Deserialize, Serialize};
use warp::{
    http::StatusCode,
    reject::Reject,
    reply::{Reply, Response},
};

use crate::{
    db::utils::{default_sort_direction, sort_direction},
    error_handler::ErrorResponse,
};
use thiserror::Error;

#[derive(Deserialize)]
pub struct Repository {
    pub id: i32,
    pub name: String,
    pub organization_id: i32,
    pub icon: String,
    pub url: String,
    pub e_tag: String,
}
#[derive(Serialize, Deserialize)]
pub struct NewRepository {
    pub name: String,
    pub icon: String,
    pub organization_id: i32,
    pub url: String,
    pub e_tag: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RepositoryResponse {
    pub id: i32,
    pub name: String,
    pub organization_id: i32,
    pub icon: String,
    pub url: String,
    pub e_tag: String,
}

impl RepositoryResponse {
    pub fn of(repository: Repository) -> RepositoryResponse {
        RepositoryResponse {
            id: repository.id,
            name: repository.name,
            organization_id: repository.organization_id,
            icon: repository.icon,
            url: repository.url,
            e_tag: repository.e_tag,
        }
    }
}

#[derive(Default)]
pub struct RepositoriesRelations {
    pub issues: bool,
    pub tips: bool,
    pub maintainers: bool,
    pub languages: bool,
}
// query args

#[derive(Serialize, Deserialize, Default)]
pub struct GetRepositoryQuery {
    pub languages: Option<bool>,
    pub tips: Option<bool>,
    pub maintainers: Option<bool>,
    pub issues: Option<bool>,
    // TODO: add filters
    // pub is_maintainer: Option<bool>,
    // pub has_tips: Option<bool>,
    // pub has_issues: Option<bool>,
    // pub has_wishes: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct RepositorySort {
    pub field: String,
    pub order: String,
}
impl RepositorySort {
    pub fn new(field: &str, descending: bool) -> Result<Self, RepositorySortError> {
        if field != "id" && field != "name" {
            return Err(RepositorySortError::InvalidSortBy(field.to_owned()));
        }

        Ok(Self {
            field: format!("repository.{field}"),
            order: sort_direction(descending),
        })
    }
}

impl Default for RepositorySort {
    fn default() -> Self {
        RepositorySort {
            field: "id".to_string(),
            order: default_sort_direction(),
        }
    }
}

#[derive(Clone, Error, Debug, Deserialize, PartialEq)]
pub enum RepositorySortError {
    InvalidSortBy(String),
}

impl fmt::Display for RepositorySortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositorySortError::InvalidSortBy(field) => {
                write!(f, "Sort by {} is invalid", field)
            }
        }
    }
}

impl Reject for RepositorySortError {}

impl Reply for RepositorySortError {
    fn into_response(self) -> Response {
        let status_code = match self {
            RepositorySortError::InvalidSortBy(_) => StatusCode::BAD_REQUEST,
        };
        let code = status_code;
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
