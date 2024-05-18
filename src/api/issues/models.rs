use crate::schema::issues;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use serde_derive::{Deserialize, Serialize};

#[derive(
    AsChangeset, Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = issues)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Issue {
    pub id: i32,
    pub number: i32,
    pub title: String,
    pub labels: Option<Vec<Option<String>>>,
    pub open: bool,
    pub assignee_id: Option<i32>,
    pub e_tag: String,
    pub repository_id: i32,
    pub issue_created_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = issues)]
pub struct NewIssue {
    pub id: i32,
    pub number: i32,
    pub title: String,
    pub labels: Option<Vec<String>>,
    pub open: bool,
    pub repository_id: i32,
    pub assignee_id: Option<i32>,
    pub e_tag: String,
    pub issue_created_at: DateTime<Utc>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = issues)]
pub struct UpdateIssue {
    pub id: i32,
    pub number: i32,
    pub title: String,
    pub labels: Option<Vec<String>>,
    pub open: bool,
    pub repository_id: Option<i32>,
    pub assignee_id: Option<i32>,
    pub e_tag: String,
    pub issue_created_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub labels: Option<String>,
    pub repository_id: Option<i32>,
    pub assignee_id: Option<i32>,
    pub open: Option<bool>,
}
