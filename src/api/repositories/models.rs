use crate::{api::projects::models::ProjectResponse, schema::repositories};
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
    pub name: String,
    pub url: String,
    pub language_slug: Option<String>,
    pub project_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub slugs: Option<String>,
    pub names: Option<String>,
    pub languages: Option<String>,
    pub project_ids: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct LanguageQueryParams {
    pub labels: Option<String>,
    pub with_technologies: Option<bool>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = repositories)]
pub struct NewRepository {
    pub slug: String,
    pub name: String,
    pub url: String,
    pub language_slug: Option<String>,
    pub project_id: i32,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = repositories)]
pub struct UpdateRepository {
    pub slug: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub language_slug: Option<String>,
    pub project_id: Option<i32>,
}

#[derive(Serialize, Debug)]
pub struct RepositoryResponse {
    pub id: i32,
    pub slug: String,
    pub name: String,
    pub url: String,
    pub language_slug: Option<String>,
    pub project: ProjectResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
