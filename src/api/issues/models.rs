use crate::{api::repositories::models::RepositoryResponse, schema::issues};
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
    pub issue_closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub estimation: i32,
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
    pub description: Option<String>,
    pub estimation: Option<i32>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Default)]
#[diesel(table_name = issues)]
pub struct UpdateIssue {
    pub title: Option<String>,
    pub labels: Option<Vec<String>>,
    pub open: Option<bool>,
    pub certified: Option<bool>,
    pub assignee_id: Option<i32>,
    pub issue_closed_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub estimation: Option<i32>,
}

impl UpdateIssue {
    pub fn has_any_field(&self) -> bool {
        self.title.is_some()
            || self.labels.is_some()
            || self.open.is_some()
            || self.certified.is_some()
            || self.assignee_id.is_some()
            || self.issue_closed_at.is_some()
            || self.description.is_some()
            || self.estimation.is_some()
    }
}

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub slugs: Option<String>,
    pub certified: Option<bool>,
    pub purposes: Option<String>,
    pub stack_levels: Option<String>,
    pub technologies: Option<String>,
    pub labels: Option<String>,
    pub language_slugs: Option<String>,
    pub repository_id: Option<i32>,
    pub assignee_id: Option<i32>,
    pub open: Option<bool>,
    pub has_assignee: Option<bool>,
    pub issue_closed_at_min: Option<DateTime<Utc>>,
    pub issue_closed_at_max: Option<DateTime<Utc>>,
    pub rewards: Option<bool>,
    pub certified_or_labels: Option<bool>,
    pub types: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueAssignee {
    pub username: String,
}

#[derive(Serialize, Debug)]
pub struct IssueResponse {
    pub id: i32,
    pub issue_id: i32,
    pub title: String,
    pub labels: Option<Vec<Option<String>>>,
    pub open: bool,
    pub certified: bool,
    pub assignee_id: Option<i32>,
    pub assignee_username: Option<String>,
    pub assignee_avatar: Option<String>,
    pub repository: RepositoryResponse,
    pub issue_created_at: DateTime<Utc>,
    pub issue_closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub estimation: i32,
}

#[derive(Serialize, Debug)]
pub struct LeaderboardEntry {
    pub username: String,
    pub issues: u64,
    pub score: u64,
}

#[derive(Deserialize, Debug)]
pub struct LeaderboardQueryParams {
    pub slugs: Option<String>,
    pub certified: Option<bool>,
    pub purposes: Option<String>,
    pub stack_levels: Option<String>,
    pub technologies: Option<String>,
    pub labels: Option<String>,
    pub language_slug: Option<String>,
    pub repository_id: Option<i32>,
    pub start_date: Option<DateTime<Utc>>,
    pub close_date: Option<DateTime<Utc>>,
    pub rewards: Option<bool>,
    pub types: Option<String>,
}
