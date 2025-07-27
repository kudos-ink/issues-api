use crate::schema::task_comments;
use crate::api::users::models::User;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Selectable, Debug, Serialize, Associations)]
#[diesel(table_name = task_comments)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub task_id: i32,
    pub user_id: i32,
    pub parent_comment_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub status: String,
}

#[derive(Serialize, Debug)]
pub struct CommentResponse {
    pub id: i32,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub user: User,
    pub parent_comment_id: Option<i32>,
    pub status: String,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = task_comments)]
pub struct NewComment {
    pub content: String,
    pub task_id: i32,
    pub user_id: i32,
    pub parent_comment_id: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct CreateCommentPayload {
    pub content: String,
    pub task_id: i32,
    pub parent_comment_id: Option<i32>,
}