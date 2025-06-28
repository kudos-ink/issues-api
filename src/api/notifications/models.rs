use crate::schema::notifications;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use serde::{Deserialize, Serialize};
use crate::api::tasks::models::TaskResponse;

#[derive(
    AsChangeset,
    Queryable,
    Identifiable,
    Selectable,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    Clone,
)]
#[diesel(table_name = notifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Notification {
    pub id: i32,
    pub github_id: i64,
    pub task_id: i32,
    pub seen: Option<bool>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = notifications)]
pub struct DeleteNotification {
    pub id: i32,
    pub github_id: Option<i64>,
}

#[derive(Serialize, Debug)]
pub struct NotificationResponse {
    pub id: i32,
    pub task_id: i32,
    pub task: TaskResponse,
    pub created_at: DateTime<Utc>,
}