use crate::schema::repositories;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use serde_derive::{Deserialize, Serialize};

#[derive(
    AsChangeset, Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = repositories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Repository {
    pub id: i32,
    pub slug: String,
    pub language_slug: String,
    pub project_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub slug: Option<String>,
    pub name: Option<String>,
    pub languages: Option<String>,
    pub project_ids: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = repositories)]
pub struct NewRepository {
    pub slug: String,
    pub language_slug: String,
    pub project_id: i32,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = repositories)]
pub struct UpdateRepository {
    pub slug: Option<String>,
    pub language_slug: Option<String>,
    pub project_id: Option<i32>,
}
