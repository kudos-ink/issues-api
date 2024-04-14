use std::default;

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
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

#[derive(Serialize, Deserialize, Default, Clone)]
pub enum UserSort {
    #[default]
    ById,
    ByUsername,
    ByCreatedAt,
}

impl UserSort {
    pub fn to_string(&self) -> &str {
        match self {
            UserSort::ById => "id",
            UserSort::ByUsername => "username",
            UserSort::ByCreatedAt => "created_at",
        }
    }
}
impl Default for GetUsersFilters {
    fn default() -> Self {
        GetUsersFilters {
            limit: Some(1000),
            offset: Some(0),
            sort: Some("users.id".to_string()),
            ascending: Some(true),
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
pub struct UsersFilters {
    pub limit: i64,
    pub offset: i64,
    pub sort: String,
    pub ascending: String,
}
// query args

#[derive(Serialize, Deserialize, Default)]
pub struct GetUserQuery {
    pub wishes: Option<bool>,
    pub tips: Option<bool>,
    pub maintainers: Option<bool>,
    pub issues: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GetUsersFilters {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub sort: Option<String>,
    pub ascending: Option<bool>,
}

impl GetUsersFilters {
    // A method to create an instance of GetUsersFilters with default values
    pub fn new() -> Self {
        GetUsersFilters::default()
    }

    // A method to set default values for individual fields if they are None
    pub fn apply_defaults(&self) -> Self {
        let mut filters = self.clone();
        let default = Self::default();
        if self.limit.is_none() {
            filters.limit = default.limit;
        }
        if self.offset.is_none() {
            filters.offset = default.offset;
        }
        if self.sort.is_none() {
            filters.sort = default.sort;
        }
        if self.ascending.is_none() {
            filters.ascending = default.ascending;
        }
        filters
    }
}
