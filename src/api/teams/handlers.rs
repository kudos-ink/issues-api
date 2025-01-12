use bytes::Buf;
use log::{error, info, warn};
use warp::{
    http::StatusCode,
    reject,
    reply::{json, with_status, Reply},
    Rejection,
};
use super::db::{DBTeam, DBTeamMembership};
use super::models::{NewTeam, UpdateTeam, NewTeamMembership, NewTeamMembershipPayload, UpdateTeamMembershipRole};

use super::errors::TeamError;


pub async fn get_all_teams(db_access: impl DBTeam) -> Result<impl Reply, Rejection> {
    info!("Fetching all teams");
    match db_access.all() {
        Ok(teams) => Ok(json(&teams)),
        Err(error) => {
            error!("Error fetching teams: {:?}", error);
            Err(reject::custom(TeamError::CannotCreate(format!(
                "Failed to fetch teams: {error}"
            ))))
        }
    }
}
pub async fn get_team_by_id(id: i32, db_access: impl DBTeam) -> Result<impl Reply, Rejection> {
    info!("Fetching team with id '{}'", id);
    match db_access.by_id(id) {
        Ok(Some(team)) => Ok(json(&team)),
        Ok(None) => {
            warn!("Team with id '{}' not found", id);
            Err(warp::reject::not_found())
        }
        Err(error) => {
            error!("Error fetching team: {:?}", error);
            Err(reject::custom(TeamError::CannotCreate(format!(
                "Failed to fetch team: {error}"
            ))))
        }
    }
}

pub async fn create_team(
    buf: impl Buf,
    db_access: impl DBTeam,
) -> Result<impl Reply, Rejection> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let team: NewTeam = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("Invalid team payload: '{}'", e);
        reject::custom(TeamError::InvalidPayload(e))
    })?;

    info!("Creating team '{}'", team.name);
    match db_access.create(&team) {
        Ok(team) => Ok(with_status(json(&team), StatusCode::CREATED)),
        Err(error) => {
            error!("Error creating team: {:?}", error);
            Err(reject::custom(TeamError::CannotCreate(format!(
                "Failed to create team: {error}"
            ))))
        }
    }
}

pub async fn update_team(
    id: i32,
    buf: impl Buf,
    db_access: impl DBTeam,
) -> Result<impl Reply, Rejection> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let updates: UpdateTeam = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("Invalid team update payload: '{}'", e);
        reject::custom(TeamError::InvalidPayload(e))
    })?;

    info!("Updating team with id '{}'", id);
    match db_access.update(id, &updates) {
        Ok(team) => Ok(json(&team)),
        Err(error) => {
            error!("Error updating team: {:?}", error);
            Err(reject::custom(TeamError::CannotUpdate(format!(
                "Failed to update team: {error}"
            ))))
        }
    }
}

pub async fn delete_team(id: i32, db_access: impl DBTeam) -> Result<impl Reply, Rejection> {
    info!("Deleting team with id '{}'", id);
    match db_access.delete(id) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(error) => {
            error!("Error deleting team: {:?}", error);
            Err(reject::custom(TeamError::CannotDelete(format!(
                "Failed to delete team: {error}"
            ))))
        }
    }
}

pub async fn add_member_to_team(
    team_id: i32,
    buf: impl Buf,
    db_access: impl DBTeamMembership,
) -> Result<impl Reply, Rejection> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let membership: NewTeamMembershipPayload = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("Invalid membership payload: '{}'", e);
        reject::custom(TeamError::InvalidPayload(e))
    })?;

    let new_membership = NewTeamMembership {
        team_id,
        user_id: membership.user_id,
        role: membership.role,
    };
    
    info!("Adding user '{}' to team '{}'", membership.user_id, team_id);
    match db_access.add_member(&new_membership) {
        Ok(member) => Ok(with_status(json(&member), StatusCode::CREATED)),
        Err(error) => {
            error!("Error adding member to team: {:?}", error);
            Err(reject::custom(TeamError::CannotCreate(format!(
                "Failed to add member to team: {error}"
            ))))
        }
    }
}

pub async fn list_team_members(
    team_id: i32,
    db_access: impl DBTeamMembership,
) -> Result<impl Reply, Rejection> {
    info!("Fetching members for team '{}'", team_id);
    match db_access.list_members(team_id) {
        Ok(members) => Ok(json(&members)),
        Err(error) => {
            error!("Error fetching team members: {:?}", error);
            Err(reject::custom(TeamError::NotFound(team_id)))
        }
    }
}

pub async fn update_member_role(
    team_id: i32,
    membership_id: i32,
    buf: impl Buf,
    db_access: impl DBTeamMembership,
) -> Result<impl Reply, Rejection> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let updates: UpdateTeamMembershipRole = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("Invalid membership update payload: '{}'", e);
        reject::custom(TeamError::InvalidPayload(e))
    })?;

    info!("Updating role for membership '{}' in team '{}'", membership_id, team_id);
    match db_access.update_member_role(membership_id, &updates) {
        Ok(member) => Ok(json(&member)),
        Err(error) => {
            error!("Error updating member role: {:?}", error);
            Err(reject::custom(TeamError::CannotUpdate(format!(
                "Failed to update member role: {error}"
            ))))
        }
    }
}

pub async fn remove_member_from_team(
    team_id: i32,
    membership_id: i32,
    db_access: impl DBTeamMembership,
) -> Result<impl Reply, Rejection> {
    info!("Removing membership '{}' from team '{}'", membership_id, team_id);
    match db_access.remove_member(membership_id) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(error) => {
            error!("Error removing member from team: {:?}", error);
            Err(reject::custom(TeamError::CannotDelete(format!(
                "Failed to remove member from team: {error}"
            ))))
        }
    }
}
