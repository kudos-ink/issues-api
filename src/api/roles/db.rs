use std::collections::HashMap;

use diesel::dsl::now;
use diesel::prelude::*;

use super::models::KudosRole;
use super::models::NewRole;
use super::models::NewUserProjectRole;
use super::models::Role;
use super::models::UpdateRole;
use super::models::UserProjectRole;
use crate::schema::roles::dsl as roles_dsl;
use crate::schema::users::dsl as users_dsl;
use crate::schema::users_projects_roles::dsl as users_projects_roles_dsl;

use crate::db::{
    errors::DBError,
    pool::{DBAccess, DBAccessor},
};
use crate::types::PaginationParams;
pub trait DBRole: Send + Sync + Clone + 'static {
    fn all(&self, pagination: PaginationParams) -> Result<(Vec<Role>, i64), DBError>;
    fn by_id(&self, id: i32) -> Result<Option<Role>, DBError>;
    fn create(&self, role: &NewRole) -> Result<Role, DBError>;
    fn update(&self, id: i32, role: &UpdateRole) -> Result<Role, DBError>;
    fn delete(&self, id: i32) -> Result<(), DBError>;
    fn create_role_to_user_and_project(
        &self,
        user_project_role: &NewUserProjectRole,
    ) -> Result<UserProjectRole, DBError>;
    fn delete_role_to_user_and_project(&self, id: i32) -> Result<(), DBError>;
    fn user_roles(&self, username: &str) -> Result<Vec<KudosRole>, DBError>;
}

impl DBRole for DBAccess {
    fn all(&self, pagination: PaginationParams) -> Result<(Vec<Role>, i64), DBError> {
        let conn = &mut self.get_db_conn();

        let build_query = || {
            let  query = roles_dsl::roles
                .into_boxed();
            query
        };

        let total_count = build_query().count().get_result::<i64>(conn)?;

        let result = build_query()
            .order(roles_dsl::created_at.desc())
            .offset(pagination.offset)
            .limit(pagination.limit)
            .load::<Role>(conn)?;

        Ok((result, total_count))
    }
    fn by_id(&self, id: i32) -> Result<Option<Role>, DBError> {
        let conn = &mut self.get_db_conn();
        let result = roles_dsl::roles
            .find(id)
            .first::<Role>(conn)
            .optional()
            .map_err(DBError::from)?;
        Ok(result)
    }

    fn create(&self, role: &NewRole) -> Result<Role, DBError> {
        let conn = &mut self.get_db_conn();
        let role = diesel::insert_into(roles_dsl::roles)
            .values(role)
            .get_result(conn)
            .map_err(DBError::from)?;

        Ok(role)
    }

    fn update(&self, id: i32, role: &UpdateRole) -> Result<Role, DBError> {
        let conn = &mut self.get_db_conn();

        let role = diesel::update(roles_dsl::roles.filter(roles_dsl::id.eq(id)))
            .set((role, roles_dsl::updated_at.eq(now)))
            .get_result::<Role>(conn)
            .map_err(DBError::from)?;

        Ok(role)
    }
    fn delete(&self, id: i32) -> Result<(), DBError> {
        let conn = &mut self.get_db_conn();
        diesel::delete(roles_dsl::roles.filter(roles_dsl::id.eq(id)))
            .execute(conn)
            .map_err(DBError::from)?;

        Ok(())
    }

    fn create_role_to_user_and_project(
        &self,
        user_project_role: &NewUserProjectRole,
    ) -> Result<UserProjectRole, DBError> {
        let conn = &mut self.get_db_conn();
        let role = diesel::insert_into(users_projects_roles_dsl::users_projects_roles)
            .values(user_project_role)
            .get_result(conn)
            .map_err(DBError::from)?;

        Ok(role)
    }

    fn delete_role_to_user_and_project(&self, id: i32) -> Result<(), DBError> {
        let conn = &mut self.get_db_conn();
        diesel::delete(
            users_projects_roles_dsl::users_projects_roles
                .filter(users_projects_roles_dsl::id.eq(id)),
        )
        .execute(conn)
        .map_err(DBError::from)?;

        Ok(())
    }

     fn user_roles(&self, username: &str) -> Result<Vec<KudosRole>, DBError> {
    let conn = &mut self.get_db_conn();

    // Build the query to join users, users_projects_roles, and roles
    let mut query = users_projects_roles_dsl::users_projects_roles
        .inner_join(
            users_dsl::users
                .on(users_projects_roles_dsl::user_id.eq(users_dsl::id)),
        )
        .inner_join(roles_dsl::roles.on(roles_dsl::id.eq(users_projects_roles_dsl::role_id)))
        .select((
            roles_dsl::id,       // Select role IDs
            users_projects_roles_dsl::project_id.nullable(), // Select project IDs (nullable)
        ))
        .into_boxed();

    // Apply filter for username (only once)
    query = query.filter(users_dsl::username.eq(username));

    // Execute the query and load the roles and project_ids
    let user_roles_with_projects: Vec<(i32, Option<i32>)> = query.load::<(i32, Option<i32>)>(conn)?;

    // Prepare the final vector of KudosRole
    let mut kudos_roles = Vec::new();
    let mut maintainer_projects: HashMap<i32, Vec<i32>> = HashMap::new();

    // Process the results
    for (role_id, project_id_opt) in user_roles_with_projects {
        match KudosRole::from_int(role_id) {
            Some(KudosRole::MaintainerWithProjects(_)) => {
                // Collect project_ids for the Maintainer role
                if let Some(project_id) = project_id_opt {
                    maintainer_projects
                        .entry(role_id)
                        .or_default()
                        .push(project_id);
                }
            }
            Some(role) => {
                if let KudosRole::MaintainerWithProjects(_) = role {
                    // If the role is MaintainerWithProjects, we need to assign project IDs
                    if let Some(project_id) = project_id_opt {
                        maintainer_projects
                            .entry(role_id)
                            .or_default()
                            .push(project_id);
                    } else {
                        kudos_roles.push(role); // Just add the role if no project ID
                    }
                } else {
                    kudos_roles.push(role); // Other roles without project IDs
                }
            }
            None => continue, // Ignore invalid role IDs
        }
    }

    // Add MaintainerWithProjects roles with the collected project IDs
    for (role_id, projects) in maintainer_projects {
        if let Some(KudosRole::MaintainerWithProjects(_)) = KudosRole::from_int(role_id) {
            kudos_roles.push(KudosRole::MaintainerWithProjects(Some(projects)));
        }
    }

    Ok(kudos_roles)
}

}
