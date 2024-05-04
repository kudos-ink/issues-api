use diesel::dsl::now;
use diesel::prelude::*;

use super::models::{NewForm, Project, QueryParams, UpdateForm};
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
    ) -> Result<Vec<Project>, DBError>;
    fn by_id(&self, id: i32) -> Result<Option<Project>, DBError>;
    fn by_slug(&self, slug: &str) -> Result<Option<Project>, DBError>;
    fn create(&self, form: &NewForm) -> Result<Project, DBError>;
    fn update(&self, id: i32, form: &UpdateForm) -> Result<Project, DBError>;
    fn delete(&self, id: i32) -> Result<(), DBError>;
}

impl DBProject for DBAccess {
    fn all(
        &self,
        params: QueryParams,
        pagination: PaginationParams,
    ) -> Result<Vec<Project>, DBError> {
        let conn = &mut self.get_db_conn();
        let mut query = projects_dsl::projects.into_boxed();

        if let Some(slug) = params.slug {
            query = query.filter(projects_dsl::slug.eq(slug));
        }

        if let Some(raw_categories) = params.categories {
            let categories: Vec<String> = utils::parse_comma_values(&raw_categories);
            query = query.filter(projects_dsl::categories.overlaps_with(categories));
        }

        if let Some(raw_purposes) = params.purposes {
            let purposes: Vec<String> = utils::parse_comma_values(&raw_purposes);
            query = query.filter(projects_dsl::purposes.overlaps_with(purposes));
        }

        if let Some(raw_technologies) = params.technologies {
            let technologies: Vec<String> = utils::parse_comma_values(&raw_technologies);
            query = query.filter(projects_dsl::technologies.overlaps_with(technologies));
        }

        query = query.offset(pagination.offset).limit(pagination.limit);

        let result = query.load::<Project>(conn)?;
        Ok(result)
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

    fn create(&self, form: &NewForm) -> Result<Project, DBError> {
        let conn = &mut self.get_db_conn();

        let project = diesel::insert_into(projects_dsl::projects)
            .values(form)
            .get_result(conn)
            .map_err(DBError::from)?;

        Ok(project)
    }

    fn update(&self, id: i32, form: &UpdateForm) -> Result<Project, DBError> {
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
