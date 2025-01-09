use crate::schema::{teams, team_memberships};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(
    AsChangeset, Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = teams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_by_user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = teams)]
pub struct NewTeam {
    pub name: String,
    pub description: Option<String>,
    pub created_by_user_id: i32,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Default)]
#[diesel(table_name = teams)]
pub struct UpdateTeam {
    pub name: Option<String>,
    pub description: Option<String>,
}



#[derive(
    AsChangeset, Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = team_memberships)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TeamMembership {
    pub id: i32,
    pub team_id: i32,
    pub user_id: i32,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = team_memberships)]
pub struct NewTeamMembership {
    pub user_id: i32,
    pub role: String,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Default)]
#[diesel(table_name = team_memberships)]
pub struct UpdateTeamMembershipRole {
    pub role: Option<String>,
}