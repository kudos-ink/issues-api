use diesel::dsl::now;
use diesel::prelude::*;

use super::models::{NewProject, Project, QueryParams, UpdateProject};
use crate::schema::projects::dsl as projects_dsl;

use crate::db::{
    errors::DBError,
    pool::{DBAccess, DBAccessor},
};
use crate::types::PaginationParams;
use crate::utils;

pub trait DBProject: Send + Sync + Clone + 'static {
    fn all(
        &self,
        params: QueryParams,
        pagination: PaginationParams,
    ) -> Result<(Vec<Project>, i64), DBError>;
    fn by_id(&self, id: i32) -> Result<Option<Project>, DBError>;
    fn by_slug(&self, slug: &str) -> Result<Option<Project>, DBError>;
    fn create(&self, form: &NewProject) -> Result<Project, DBError>;
    fn update(&self, id: i32, form: &UpdateProject) -> Result<Project, DBError>;
    fn delete(&self, id: i32) -> Result<(), DBError>;
}

impl DBProject for DBAccess {
    fn all(
        &self,
        params: QueryParams,
        pagination: PaginationParams,
    ) -> Result<(Vec<Project>, i64), DBError> {
        let conn = &mut self.get_db_conn();

        let build_query = || {
            let mut query = projects_dsl::projects.into_boxed();

            if let Some(slug) = params.slug.as_ref() {
                query = query.filter(projects_dsl::slug.eq(slug));
            }

            if let Some(raw_types) = params.types.as_ref() {
                let types: Vec<String> = utils::parse_comma_values(raw_types);
                query = query.filter(projects_dsl::types.overlaps_with(types));
            }

            if let Some(raw_purposes) = params.purposes.as_ref() {
                let purposes: Vec<String> = utils::parse_comma_values(raw_purposes);
                query = query.filter(projects_dsl::purposes.overlaps_with(purposes));
            }

            if let Some(raw_technologies) = params.technologies.as_ref() {
                let technologies: Vec<String> = utils::parse_comma_values(raw_technologies);
                query = query.filter(projects_dsl::technologies.overlaps_with(technologies));
            }

            query
        };

        let total_count = build_query().count().get_result::<i64>(conn)?;

        let result = build_query()
            .offset(pagination.offset)
            .limit(pagination.limit)
            .load::<Project>(conn)?;

        Ok((result, total_count))
    }

    fn by_id(&self, id: i32) -> Result<Option<Project>, DBError> {
        let conn = &mut self.get_db_conn();

        let result = projects_dsl::projects
            .find(id)
            .first::<Project>(conn)
            .optional()
            .map_err(DBError::from)?;

        Ok(result)
    }

    fn by_slug(&self, slug: &str) -> Result<Option<Project>, DBError> {
        let conn = &mut self.get_db_conn();

        let result = projects_dsl::projects
            .filter(projects_dsl::slug.eq(slug))
            .first::<Project>(conn)
            .optional()
            .map_err(DBError::from)?;

        Ok(result)
    }

    fn create(&self, form: &NewProject) -> Result<Project, DBError> {
        let conn = &mut self.get_db_conn();

        let project = diesel::insert_into(projects_dsl::projects)
            .values(form)
            .get_result(conn)
            .map_err(DBError::from)?;

        Ok(project)
    }

    fn update(&self, id: i32, form: &UpdateProject) -> Result<Project, DBError> {
        let conn = &mut self.get_db_conn();

        let project = diesel::update(projects_dsl::projects.filter(projects_dsl::id.eq(id)))
            .set((form, projects_dsl::updated_at.eq(now)))
            .get_result::<Project>(conn)
            .map_err(DBError::from)?;

        Ok(project)
    }

    fn delete(&self, id: i32) -> Result<(), DBError> {
        let conn = &mut self.get_db_conn();
        diesel::delete(projects_dsl::projects.filter(projects_dsl::id.eq(id)))
            .execute(conn)
            .map_err(DBError::from)?;

        Ok(())
    }
}
