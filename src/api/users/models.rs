use crate::schema::users;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use serde_derive::{Deserialize, Serialize};

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
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub avatar: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub github_id: Option<i64>,
    pub email_notifications_enabled: bool,
    pub email: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub github_id: Option<i64>,

}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub avatar: Option<String>,
    pub github_id: Option<i64>,
    pub email_notifications_enabled: Option<bool>,

}
#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct UpdateEmailNotificationsUser {
    pub email_notifications_enabled: Option<bool>,

}
#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub labels: Option<String>,
    pub certified: Option<bool>,
}