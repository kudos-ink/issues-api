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
    pub certified: Option<bool>,
    pub assignee_id: Option<i32>,
    pub repository_id: i32,
    pub issue_created_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = issues)]
pub struct NewIssue {
    pub number: i32,
    pub title: String,
    pub labels: Option<Vec<String>>,
    pub open: bool,
    pub certified: Option<bool>,
    pub repository_id: i32,
    pub assignee_id: Option<i32>,
    pub issue_created_at: DateTime<Utc>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Default)]
#[diesel(table_name = issues)]
pub struct UpdateIssue {
    pub title: Option<String>,
    pub labels: Option<Vec<String>>,
    pub open: Option<bool>,
    pub certified: Option<bool>,
    pub assignee_id: Option<i32>,
}

impl UpdateIssue {
    pub fn has_any_field(&self) -> bool {
        self.title.is_some()
            || self.labels.is_some()
            || self.open.is_some()
            || self.certified.is_some()
            || self.assignee_id.is_some()
    }
}

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub slug: Option<String>,
    pub categories: Option<String>,
    pub purposes: Option<String>,
    pub stack_levels: Option<String>,
    pub technologies: Option<String>,
    pub labels: Option<String>,
    pub language_slug: Option<String>,
    pub repository_id: Option<i32>,
    pub assignee_id: Option<i32>,
    pub open: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueAssignee {
    pub username: String,
}
