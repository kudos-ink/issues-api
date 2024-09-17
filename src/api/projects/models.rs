use crate::schema::projects;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use serde_derive::{Deserialize, Serialize};

#[derive(
    AsChangeset, Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub types: Option<Vec<Option<String>>>,
    pub purposes: Option<Vec<Option<String>>>,
    pub stack_levels: Option<Vec<Option<String>>>,
    pub technologies: Option<Vec<Option<String>>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub slug: Option<String>,
    pub purposes: Option<String>,
    pub stack_levels: Option<String>,
    pub technologies: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = projects)]
pub struct NewProject {
    pub name: String,
    pub slug: String,
    pub purposes: Option<Vec<Option<String>>>,
    pub stack_levels: Option<Vec<Option<String>>>,
    pub technologies: Option<Vec<Option<String>>>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = projects)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub purposes: Option<Vec<Option<String>>>,
    pub stack_levels: Option<Vec<Option<String>>>,
    pub technologies: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Debug)]
pub struct ProjectResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub purposes: Option<Vec<Option<String>>>,
    pub stack_levels: Option<Vec<Option<String>>>,
    pub technologies: Option<Vec<Option<String>>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
