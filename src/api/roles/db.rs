use diesel::dsl::now;
use diesel::prelude::*;

use super::models::NewRole;
use super::models::NewUserProjectRole;
use super::models::Role;
use super::models::UpdateRole;
use super::models::UserProjectRole;
use crate::schema::roles::dsl as roles_dsl;
use crate::schema::users_projects_roles::dsl as users_projects_roles_dsl;

use crate::db::{
    errors::DBError,
    pool::{DBAccess, DBAccessor},
};
use crate::types::PaginationParams;
pub trait DBRole: Send + Sync + Clone + 'static {
    fn all(
        &self,
        pagination: PaginationParams,
    ) -> Result<(Vec<Role>, i64), DBError>;
    fn by_id(&self, id: i32) -> Result<Option<Role>, DBError>;
    fn create(&self, role: &NewRole) -> Result<Role, DBError>;
    fn update(&self, id: i32, role: &UpdateRole) -> Result<Role, DBError>;
    fn delete(&self, id: i32) -> Result<(), DBError>;
    fn create_role_to_user_and_project(&self, user_project_role: &NewUserProjectRole) -> Result<UserProjectRole, DBError>;
    fn delete_role_to_user_and_project(&self, id: i32) -> Result<(), DBError>;
}

impl DBRole for DBAccess {
    fn all(
        &self,
        pagination: PaginationParams,
    ) -> Result<(Vec<Role>, i64), DBError> {
        let conn = &mut self.get_db_conn();

        let build_query = || {
            let mut query = roles_dsl::roles
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

    fn create_role_to_user_and_project(&self, user_project_role: &NewUserProjectRole) -> Result<UserProjectRole, DBError> {
        let conn = &mut self.get_db_conn();
        let role = diesel::insert_into(users_projects_roles_dsl::users_projects_roles)
            .values(user_project_role)
            .get_result(conn)
            .map_err(DBError::from)?;

        Ok(role)
    }

    fn delete_role_to_user_and_project(&self, id: i32) -> Result<(), DBError> {
        let conn = &mut self.get_db_conn();
        diesel::delete(users_projects_roles_dsl::users_projects_roles.filter(users_projects_roles_dsl::id.eq(id)))
            .execute(conn)
            .map_err(DBError::from)?;

        Ok(())
    }
}
