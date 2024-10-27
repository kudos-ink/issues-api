use diesel::dsl::now;
use diesel::prelude::*;

use super::models::{NewProject, Project, QueryParams, UpdateProject};
use crate::schema::projects::dsl as projects_dsl;
use crate::schema::issues::dsl as issues_dsl;
use crate::schema::repositories::dsl as repositories_dsl;

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
        
        // filter by labels
        let project_ids: Option<Vec<i32>> = if let Some(labels) = params.labels.as_ref() {
            let label_ids = utils::parse_comma_values(labels);
    
            issues_dsl::issues
                .inner_join(
                    repositories_dsl::repositories
                        .on(issues_dsl::repository_id.eq(repositories_dsl::id)),
                )
                .inner_join(
                    projects_dsl::projects
                        .on(repositories_dsl::project_id.eq(projects_dsl::id)),
                )
                .select(projects_dsl::id)
                .filter(issues_dsl::labels.overlaps_with(label_ids))
                .distinct()
                .load::<i32>(conn)
                .optional()?
    
        } else {
            None
        };
    
        let build_query = || {
            let mut query = projects_dsl::projects.into_boxed();
            if let Some(slugs) = params.slugs.as_ref() {
                query = query.filter(projects_dsl::slug.eq_any(utils::parse_comma_values(slugs)));
            }
            if let Some(purposes) = params.purposes.as_ref() {
                query = query.filter(projects_dsl::purposes.overlaps_with( utils::parse_comma_values(purposes)));
            }
            if let Some(technologies) = params.technologies.as_ref() {
                query = query.filter(projects_dsl::technologies.overlaps_with(utils::parse_comma_values(technologies)));
            }
            if let Some(stack_levels) = params.stack_levels.as_ref() {
                    query = query.filter(projects_dsl::stack_levels.overlaps_with(utils::parse_comma_values(stack_levels)));
            }
            if let Some(rewards) = params.rewards.as_ref() {
                    query = query.filter(projects_dsl::rewards.eq(rewards));
            }

            if let Some(project_ids) = project_ids.as_ref() {
                query = query.filter(projects_dsl::id.eq_any(project_ids));
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
