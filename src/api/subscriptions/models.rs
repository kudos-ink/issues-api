use crate::schema::user_subscriptions;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

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
#[diesel(table_name = user_subscriptions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserSubscription {
    pub id: i32,
    pub github_id: i64,
    pub purpose: Option<String>,
    pub stack_level: Option<String>,
    pub technology: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_subscriptions)]
pub struct NewUserSubscription {
    pub github_id: Option<i64>,
    pub purpose: Option<String>,
    pub stack_level: Option<String>,
    pub technology: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_subscriptions)]
pub struct DeleteUserSubscription {
    pub github_id: Option<i64>,
    pub purpose: Option<String>,
    pub stack_level: Option<String>,
    pub technology: Option<String>,
}
