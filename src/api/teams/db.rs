use diesel::dsl::now;
use diesel::prelude::*;

use super::models::{Team, NewTeam, UpdateTeam, TeamMembership, NewTeamMembership, UpdateTeamMembershipRole};
use crate::schema::{teams, team_memberships};
use crate::db::{
    errors::DBError,
    pool::{DBAccess, DBAccessor},
};

pub trait DBTeam: Send + Sync + Clone + 'static {
    fn all(&self) -> Result<Vec<Team>, DBError>;
    fn by_id(&self, id: i32) -> Result<Option<Team>, DBError>;
    fn create(&self, team: &NewTeam) -> Result<Team, DBError>;
    fn update(&self, id: i32, updates: &UpdateTeam) -> Result<Team, DBError>;
    fn delete(&self, id: i32) -> Result<(), DBError>;
}

impl DBTeam for DBAccess {
    fn all(&self) -> Result<Vec<Team>, DBError> {
        let conn = &mut self.get_db_conn();
        let teams = teams::table
            .load::<Team>(conn)
            .map_err(DBError::from)?;
        Ok(teams)
    }

    fn by_id(&self, id: i32) -> Result<Option<Team>, DBError> {
        let conn = &mut self.get_db_conn();
        let team = teams::table
            .find(id)
            .first::<Team>(conn)
            .optional()
            .map_err(DBError::from)?;
        Ok(team)
    }

    fn create(&self, team: &NewTeam) -> Result<Team, DBError> {
        let conn = &mut self.get_db_conn();
        let team = diesel::insert_into(teams::table)
            .values(team)
            .get_result(conn)
            .map_err(DBError::from)?;
        Ok(team)
    }

    fn update(&self, id: i32, updates: &UpdateTeam) -> Result<Team, DBError> {
        let conn = &mut self.get_db_conn();
        let team = diesel::update(teams::table.filter(teams::id.eq(id)))
            .set((updates, teams::updated_at.eq(now)))
            .get_result::<Team>(conn)
            .map_err(DBError::from)?;
        Ok(team)
    }

    fn delete(&self, id: i32) -> Result<(), DBError> {
        let conn = &mut self.get_db_conn();
        diesel::delete(teams::table.filter(teams::id.eq(id)))
            .execute(conn)
            .map_err(DBError::from)?;
        Ok(())
    }
}


pub trait DBTeamMembership: Send + Sync + Clone + 'static {
    fn add_member(&self, membership: &NewTeamMembership) -> Result<TeamMembership, DBError>;
    fn list_members(&self, team_id: i32) -> Result<Vec<TeamMembership>, DBError>;
    fn update_member_role(&self, membership_id: i32, updates: &UpdateTeamMembershipRole) -> Result<TeamMembership, DBError>;
    fn remove_member(&self, membership_id: i32) -> Result<(), DBError>;
}

impl DBTeamMembership for DBAccess {
    fn add_member(&self, membership: &NewTeamMembership) -> Result<TeamMembership, DBError> {
        let conn = &mut self.get_db_conn();
        let member = diesel::insert_into(team_memberships::table)
            .values(membership)
            .get_result::<TeamMembership>(conn)
            .map_err(DBError::from)?;
        Ok(member)
    }

    fn list_members(&self, team_id: i32) -> Result<Vec<TeamMembership>, DBError> {
        let conn = &mut self.get_db_conn();
        let members = team_memberships::table
            .filter(team_memberships::team_id.eq(team_id))
            .load::<TeamMembership>(conn)
            .map_err(DBError::from)?;
        Ok(members)
    }

    fn update_member_role(&self, membership_id: i32, updates: &UpdateTeamMembershipRole) -> Result<TeamMembership, DBError> {
        let conn = &mut self.get_db_conn();
        let member = diesel::update(team_memberships::table.filter(team_memberships::id.eq(membership_id)))
            .set(updates)
            .get_result::<TeamMembership>(conn)
            .map_err(DBError::from)?;
        Ok(member)
    }

    fn remove_member(&self, membership_id: i32) -> Result<(), DBError> {
        let conn = &mut self.get_db_conn();
        diesel::delete(team_memberships::table.filter(team_memberships::id.eq(membership_id)))
            .execute(conn)
            .map_err(DBError::from)?;
        Ok(())
    }
}
