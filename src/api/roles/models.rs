use crate::schema::{roles,users_projects_roles };
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use serde_derive::{Deserialize, Serialize};
// roles
#[derive(
    AsChangeset, Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub name: String,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Default)]
#[diesel(table_name = roles)]
pub struct UpdateRole {
    pub name: String,
}

#[derive(Serialize, Debug)]
pub struct RoleResponse {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
// user project role

// roles
#[derive(
    AsChangeset, Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = users_projects_roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserProjectRole {
    pub id: i32,
    pub user_id: i32,
    pub project_id: i32,
    pub role_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users_projects_roles)]
pub struct NewUserProjectRole {
    pub user_id: i32,
    pub project_id: i32,
    pub role_id: i32,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Default)]
#[diesel(table_name = users_projects_roles)]
pub struct UpdateUserProjectRole {
    pub user_id: i32,
    pub project_id: i32,
    pub role_id: i32,
}

#[derive(Serialize, Debug)]
pub struct UserProjectRoleResponse {
    pub user_id: i32,
    pub project_id: i32,
    pub role_id: i32,
}
