use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
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
