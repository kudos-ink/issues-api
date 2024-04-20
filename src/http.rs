use std::fmt;

use super::db::utils::detect_sql_injection;
use crate::handlers::ErrorResponse;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use warp::{
    http::StatusCode,
    reject::Reject,
    reply::{Reply, Response},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct GetPagination {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    // pub sort: Option<String>,
    // pub ascending: Option<bool>,
}

impl GetPagination {
    // A method to set default values for individual fields if they are None
    pub fn validate(&self) -> Result<Self, PaginationError> {
        let mut filters = self.clone();
        let default = Self::default();

        if self.limit.is_none() {
            filters.limit = default.limit;
        } else {
            let limit = self.limit.unwrap();
            if limit <= 0 || limit >= 1000 {
                return Err(PaginationError::InvalidLimit(limit));
            }
        }
        if self.offset.is_none() {
            filters.offset = default.offset;
        } else {
            let offset = self.offset.unwrap();
            if offset <= 0 {
                return Err(PaginationError::InvalidOffset(offset));
            }
        }
        // if self.sort.is_none() {
        //     filters.sort = default.sort;
        // }
        // if self.ascending.is_none() {
        //     filters.ascending = default.ascending;
        // }
        Ok(filters)
    }
}

impl Default for GetPagination {
    fn default() -> Self {
        GetPagination {
            limit: Some(1000),
            offset: Some(0),
            // sort: Some("users.id".to_string()),
            // ascending: Some(true),
        }
    }
}

#[derive(Clone, Error, Debug, Deserialize, PartialEq)]
pub enum PaginationError {
    InvalidOffset(i64),
    InvalidLimit(i64),
}

impl fmt::Display for PaginationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PaginationError::InvalidOffset(offset) => {
                write!(f, "Offset {} is invalid", offset)
            }
            PaginationError::InvalidLimit(limit) => {
                write!(f, "Limit #{} not found", limit)
            }
        }
    }
}

impl Reject for PaginationError {}

impl Reply for PaginationError {
    fn into_response(self) -> Response {
        let code = match self {
            PaginationError::InvalidOffset(_) => StatusCode::BAD_REQUEST,
            PaginationError::InvalidLimit(_) => StatusCode::BAD_REQUEST,
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct GetSort {
    pub sort_by: Option<String>,
    pub descending: Option<bool>,
}

#[derive(Clone, Error, Debug, Deserialize, PartialEq)]
pub enum SortError {
    InvalidSortBy,
}

impl GetSort {
    pub fn validate(&self) -> Result<Self, SortError> {
        match (self.sort_by.clone(), self.descending) {
            (None, None) => Ok(self.clone()),
            (None, Some(_)) => Ok(Self {
                sort_by: None,
                descending: None,
            }),
            (Some(sort_by), some_or_none) => {
                //TODO: improve with trim, remove unexpected chars, etc.
                if detect_sql_injection(&sort_by) {
                    Err(SortError::InvalidSortBy)
                } else {
                    Ok(Self {
                        sort_by: Some(sort_by),
                        descending: if some_or_none.is_none() {
                            Some(false)
                        } else {
                            some_or_none
                        },
                    })
                }
            }
        }
    }
}

impl fmt::Display for SortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortError::InvalidSortBy => {
                write!(f, "Sort by is invalid")
            }
        }
    }
}

impl Reject for SortError {}

impl Reply for SortError {
    fn into_response(self) -> Response {
        let code = match self {
            SortError::InvalidSortBy => StatusCode::BAD_REQUEST,
        };
        let message = self.to_string();

        let json = warp::reply::json(&ErrorResponse { message });

        warp::reply::with_status(json, code).into_response()
    }
}
