use crate::schema::{tasks, tasks_votes};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use crate::api::users::models::User;
use crate::api::repositories::models::RepositoryResponse;

use serde_derive::{Deserialize, Serialize};
// tasks
#[derive(
    AsChangeset, Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    pub id: i32,
    pub number: Option<i32>,
    pub repository_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub labels: Option<Vec<Option<String>>>,
    pub open: bool,
    pub type_: String, // Maps to `type` in the database, renamed to `type_` to avoid keyword conflict
    pub project_id: Option<i32>,
    pub created_by_user_id: Option<i32>,
    pub assignee_user_id: Option<i32>,
    pub assignee_team_id: Option<i32>,
    pub funding_options: Option<Vec<Option<String>>>,
    pub contact: Option<String>,
    pub skills: Option<Vec<Option<String>>>,
    pub bounty: Option<i32>,
    pub approved_by: Option<Vec<Option<i32>>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub status: String,
    pub upvotes: Option<i32>,
    pub downvotes: Option<i32>,
    pub is_featured: Option<bool>,
    pub is_certified: Option<bool>,
    pub featured_by_user_id: Option<i32>,
    pub issue_created_at: Option<DateTime<Utc>>,
    pub issue_closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub repository_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub labels: Option<Vec<Option<String>>>,
    pub open: Option<bool>,
    pub type_: String, // Maps to `type` in the database, renamed to `type_` to avoid keyword conflict
    pub project_id: Option<i32>,
    pub created_by_user_id: Option<i32>,
    pub assignee_user_id: Option<i32>,
    pub assignee_team_id: Option<i32>,
    pub funding_options: Option<Vec<Option<String>>>,
    pub contact: Option<String>,
    pub skills: Option<Vec<Option<String>>>,
    pub bounty: Option<i32>,
    pub approved_by: Option<Vec<Option<i32>>>,
    pub status: Option<String>,
    pub is_featured: Option<bool>,
    pub is_certified: Option<bool>,
    pub featured_by_user_id: Option<i32>,
    pub issue_created_at: Option<DateTime<Utc>>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Default)]
#[diesel(table_name = tasks)]
pub struct UpdateTask {
    pub repository_id: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub labels: Option<Vec<Option<String>>>,
    pub open: Option<bool>,
    pub type_: Option<String>, // Maps to `type` in the database, renamed to `type_` to avoid keyword conflict
    pub project_id: Option<i32>,
    pub assignee_user_id: Option<i32>,
    pub assignee_team_id: Option<i32>,
    pub funding_options: Option<Vec<Option<String>>>,
    pub contact: Option<String>,
    pub skills: Option<Vec<Option<String>>>,
    pub bounty: Option<i32>,
    pub approved_by: Option<Vec<Option<i32>>>,
    pub status: Option<String>,
    pub is_featured: Option<bool>,
    pub is_certified: Option<bool>,
    pub featured_by_user_id: Option<i32>,
}
impl UpdateTask {
    pub fn has_any_field(&self) -> bool {
        self.repository_id.is_some()
            || self.title.is_some() // Title is a String and can't be `None`, so check if it's not empty
            || self.description.is_some()
            || self.url.is_some()
            || self.labels.is_some()
            || self.open.is_some() // Boolean field, always has a value
            || self.type_.is_some() // Same as title, check if it's not empty
            || self.project_id.is_some()
            || self.assignee_user_id.is_some()
            || self.assignee_team_id.is_some()
            || self.funding_options.is_some()
            || self.contact.is_some()
            || self.skills.is_some()
            || self.bounty.is_some()
            || self.approved_by.is_some()
            || self.status.is_some() // Same as title and type_
            || self.is_featured.is_some()
            || self.is_certified.is_some()
            || self.featured_by_user_id.is_some()
    }
}

#[derive(Serialize, Debug)]
pub struct TaskResponse {
    pub id: i32,
    pub number: Option<i32>,
    pub repository_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub labels: Option<Vec<Option<String>>>,
    pub open: bool,
    pub type_: String, // Maps to `type` in the database, renamed to `type_` to avoid keyword conflict
    pub project_id: Option<i32>,
    pub created_by_user_id: Option<i32>,
    pub assignee_user_id: Option<i32>,
    pub user: Option<User>, // return the user if there is one already assigned
    pub repository: Option<RepositoryResponse>,
    pub assignee_team_id: Option<i32>,
    pub funding_options: Option<Vec<Option<String>>>,
    pub contact: Option<String>,
    pub skills: Option<Vec<Option<String>>>,
    pub bounty: Option<i32>,
    pub approved_by: Option<Vec<Option<i32>>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub status: String,
    pub upvotes: Option<i32>,
    pub downvotes: Option<i32>,
    pub user_vote: Option<i32>,
    pub is_featured: Option<bool>,
    pub is_certified: Option<bool>,
    pub featured_by_user_id: Option<i32>,
    pub issue_created_at: Option<DateTime<Utc>>,
    pub issue_closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub repository_id: Option<i32>,
    pub labels: Option<String>,
    pub open: Option<bool>,
    pub type_: Option<String>, // Maps to `type` in the database, renamed to `type_` to avoid keyword conflict
    pub project_id: Option<i32>,
    pub created_by_user_id: Option<i32>,
    pub assignee_user_id: Option<i32>,
    pub assignee_team_id: Option<i32>,
    pub funding_options: Option<String>,
    pub contact: Option<String>,
    pub skills: Option<String>,
    pub bounty: Option<i32>,
    // pub approved_by: Option<i32>,
    pub approved_at: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub upvotes: Option<i32>,
    pub downvotes: Option<i32>,
    pub is_featured: Option<bool>,
    pub is_certified: Option<bool>,
    pub featured_by_user_id: Option<i32>,
    pub issue_created_at: Option<DateTime<Utc>>,
    pub issue_closed_at: Option<DateTime<Utc>>,
}

// tasks
#[derive(
    AsChangeset, Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = tasks_votes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TaskVote {
    pub id: i32,
    pub user_id: i32,
    pub task_id: i32,
    pub vote: i32
}

// #[derive(Insertable, Serialize, Deserialize, Debug)]
// #[diesel(table_name = tasks_votes)]
// pub struct NewTaskVote {
//     pub user_id: i32,
//     pub task_id: i32,
// }

#[derive(Deserialize, Debug)]
pub struct VotePayload {
    pub task_id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = tasks_votes)]
pub struct TaskVoteDB {
    pub user_id: i32,
    pub task_id: i32,
    pub vote: i32
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = tasks_votes)]
pub struct UserVote {
    pub user_id: i32,
}
