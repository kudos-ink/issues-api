use std::collections::HashSet;
use warp::reject::{self, Rejection};

use super::{db::DBRole, errors::RoleError, models::KudosRole};

pub fn user_has_at_least_one_role(
    user_roles: Vec<KudosRole>,
    required_roles: Vec<KudosRole>,
) -> Result<(), Rejection> {
    // Collect the roles into a set for quick lookup
    let user_role_set: HashSet<_> = user_roles.into_iter().collect();

    // Check if any of the required roles are present
    if required_roles.iter().any(|required_role| match required_role {
        // Handle `MaintainerWithProjects`
        KudosRole::MaintainerWithProjects(None) => {
            user_role_set.iter().any(|user_role| matches!(user_role, KudosRole::MaintainerWithProjects(Some(_))))
        }
        KudosRole::MaintainerWithProjects(Some(required_projects)) => {
            user_role_set.iter().any(|user_role| {
                if let KudosRole::MaintainerWithProjects(Some(user_projects)) = user_role {
                    required_projects.iter().any(|project_id| user_projects.contains(project_id))
                } else {
                    false
                }
            })
        }
        // Handle all other roles
        _ => user_role_set.contains(required_role),
    }) {
        Ok(()) // The user has at least one required role
    } else {
        // Find the missing roles
        let missing_roles: Vec<String> = required_roles
            .iter()
            .filter(|required_role| match required_role {
                KudosRole::MaintainerWithProjects(None) => {
                    !user_role_set.iter().any(|user_role| matches!(user_role, KudosRole::MaintainerWithProjects(Some(_))))
                }
                KudosRole::MaintainerWithProjects(Some(required_projects)) => {
                    !user_role_set.iter().any(|user_role| {
                        if let KudosRole::MaintainerWithProjects(Some(user_projects)) = user_role {
                            required_projects.iter().any(|project_id| user_projects.contains(project_id))
                        } else {
                            false
                        }
                    })
                }
                _ => !user_role_set.contains(required_role),
            })
            .map(|role| format!("{:?}", role)) // Format role to string
            .collect();

        let missing_roles_str = missing_roles.join(", "); // Join missing roles into a string
        Err(reject::custom(RoleError::MissingRole(missing_roles_str))) // Return custom error
    }
}