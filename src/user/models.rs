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

#[derive(Serialize, Deserialize)]
pub enum UserSort {
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
impl Default for UsersQuery {
    fn default() -> Self {
        UsersQuery {
            limit: Some(10),
            offset: Some(0), // sort: UserSort::ById,
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
}

#[derive(Serialize, Deserialize)]
pub struct UsersQuery {
    limit: Option<u32>,
    offset: Option<u32>,
    // sort: UserSort,
}
