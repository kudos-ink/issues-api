use mobc_postgres::tokio_postgres::Error;
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
    // TODO: add filters
    // pub is_maintainer: Option<bool>,
    // pub has_tips: Option<bool>,
    // pub has_issues: Option<bool>,
    // pub has_wishes: Option<bool>,
    // pub has_wishes: Option<bool>,
}
impl UserSort {
    pub fn new(sort_by: &str, descending: bool) -> Self {
        //TODO: validation may happen here, analyze

        Self {
            field: sort_by.to_string(),
            order: {
                if descending {
                    "DESC".to_string()
                } else {
                    "ASC".to_string()
                }
            },
        }
    }
}

impl Default for UserSort {
    fn default() -> Self {
        UserSort {
            field: "users.id".to_string(),
            order: "ASC".to_string(),
            // sort: Some("users.id".to_string()),
            // ascending: Some(true),
        }
    }
}
