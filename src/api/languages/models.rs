use crate::schema::languages;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = languages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Language {
    pub id: i32,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = languages)]
pub struct NewLanguage {
    pub slug: String,
}
