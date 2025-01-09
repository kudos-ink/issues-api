#[cfg(test)]
mod tests {
    use crate::{
        api::teams::{
            db::{DBTeam, DBTeamMembership},
            models::{NewTeam, Team, TeamMembership, NewTeamMembership, UpdateTeamMembershipRole},
            routes::routes,
        },
        errors::{ErrorResponse, TeamError},
    };
    use mobc::async_trait;
    use warp::{reject, test::request, Filter};

    #[derive(Clone)]
    pub struct TeamsDBMock {}

    #[async_trait]
    impl DBTeam for TeamsDBMock {
        async fn all(&self) -> Result<Vec<Team>, reject::Rejection> {
            Ok(vec![Team {
                id: 1,
                name: "Test Team".to_string(),
                description: Some("A mock team".to_string()),
                created_by_user_id: 1,
                created_at: chrono::Utc::now(),
                updated_at: None,
            }])
        }

        async fn by_id(&self, id: i32) -> Result<Option<Team>, reject::Rejection> {
            if id == 1 {
                Ok(Some(Team {
                    id: 1,
                    name: "Test Team".to_string(),
                    description: Some("A mock team".to_string()),
                    created_by_user_id: 1,
                    created_at: chrono::Utc::now(),
                    updated_at: None,
                }))
            } else {
                Err(reject::custom(TeamError::NotFound(id)))
            }
        }

        async fn create(&self, team: &NewTeam) -> Result<Team, reject::Rejection> {
            Ok(Team {
                id: 2,
                name: team.name.clone(),
                description: team.description.clone(),
                created_by_user_id: team.created_by_user_id,
                created_at: chrono::Utc::now(),
                updated_at: None,
            })
        }

        async fn update(&self, id: i32, updates: UpdateTeam) -> Result<Team, reject::Rejection> {
            Ok(Team {
                id,
                name: updates.name.clone().unwrap_or_else(|| "Updated Team".to_string()),
                description: updates.description.clone(),
                created_by_user_id: 1,
                created_at: chrono::Utc::now(),
                updated_at: Some(chrono::Utc::now()),
            })
        }

        async fn delete(&self, id: i32) -> Result<(), reject::Rejection> {
            if id == 1 {
                Ok(())
            } else {
                Err(reject::custom(TeamError::NotFound(id)))
            }
        }
    }

    #[async_trait]
    impl DBTeamMembership for TeamsDBMock {
        async fn add_member(&self, membership: &NewTeamMembership) -> Result<TeamMembership, reject::Rejection> {
            Ok(TeamMembership {
                id: 1,
                team_id: membership.team_id,
                user_id: membership.user_id,
                role: membership.role.clone(),
                joined_at: chrono::Utc::now(),
            })
        }

        async fn list_members(&self, team_id: i32) -> Result<Vec<TeamMembership>, reject::Rejection> {
            if team_id == 1 {
                Ok(vec![TeamMembership {
                    id: 1,
                    team_id,
                    user_id: 1,
                    role: "member".to_string(),
                    joined_at: chrono::Utc::now(),
                }])
            } else {
                Ok(vec![])
            }
        }

        async fn update_member_role(&self, membership_id: i32, updates: UpdateTeamMembershipRole) -> Result<TeamMembership, reject::Rejection> {
            Ok(TeamMembership {
                id: membership_id,
                team_id: 1,
                user_id: 1,
                role: updates.role.clone().unwrap_or_else(|| "lead".to_string()),
                joined_at: chrono::Utc::now(),
            })
        }

        async fn remove_member(&self, membership_id: i32) -> Result<(), reject::Rejection> {
            if membership_id == 1 {
                Ok(())
            } else {
                Err(reject::custom(TeamError::MemberNotFound(membership_id)))
            }
        }
    }

    #[tokio::test]
    async fn test_get_all_teams() {
        let r = routes(TeamsDBMock {}).recover(errors::error_handler);
        let resp = request().path("/teams").reply(&r).await;
        assert_eq!(resp.status(), 200);
        let body: Vec<Team> = serde_json::from_slice(&resp.into_body()).unwrap();
        assert_eq!(body.len(), 1);
        assert_eq!(body[0].name, "Test Team");
    }

    #[tokio::test]
    async fn test_get_team_by_id_found() {
        let r = routes(TeamsDBMock {}).recover(errors::error_handler);
        let resp = request().path("/teams/1").reply(&r).await;
        assert_eq!(resp.status(), 200);
        let body: Team = serde_json::from_slice(&resp.into_body()).unwrap();
        assert_eq!(body.name, "Test Team");
    }

    #[tokio::test]
    async fn test_get_team_by_id_not_found() {
        let r = routes(TeamsDBMock {}).recover(errors::error_handler);
        let resp = request().path("/teams/2").reply(&r).await;
        assert_eq!(resp.status(), 404);
        let body: ErrorResponse = serde_json::from_slice(&resp.into_body()).unwrap();
        assert_eq!(
            body,
            ErrorResponse {
                message: "Team #2 not found".to_string()
            }
        );
    }

    #[tokio::test]
    async fn test_create_team() {
        let new_team = serde_json::to_vec(&NewTeam {
            name: "New Team".to_string(),
            description: Some("A new mock team".to_string()),
            created_by_user_id: 1,
        })
        .unwrap();

        let r = routes(TeamsDBMock {}).recover(errors::error_handler);
        let resp = request()
            .path("/teams")
            .method("POST")
            .body(new_team)
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 201);
        let body: Team = serde_json::from_slice(&resp.into_body()).unwrap();
        assert_eq!(body.name, "New Team");
    }

    #[tokio::test]
    async fn test_delete_team_found() {
        let r = routes(TeamsDBMock {}).recover(errors::error_handler);
        let resp = request().path("/teams/1").method("DELETE").reply(&r).await;
        assert_eq!(resp.status(), 204);
    }

    #[tokio::test]
    async fn test_delete_team_not_found() {
        let r = routes(TeamsDBMock {}).recover(errors::error_handler);
        let resp = request().path("/teams/2").method("DELETE").reply(&r).await;
        assert_eq!(resp.status(), 404);
        let body: ErrorResponse = serde_json::from_slice(&resp.into_body()).unwrap();
        assert_eq!(
            body,
            ErrorResponse {
                message: "Team #2 not found".to_string()
            }
        );
    }
}
