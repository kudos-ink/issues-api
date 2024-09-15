use diesel::dsl::now;
use diesel::prelude::*;
use diesel::sql_query;

use super::models::{Issue, NewIssue, QueryParams, UpdateIssue};
use crate::schema::issues::dsl as issues_dsl;
use crate::schema::languages::dsl as languages_dsl;
use crate::schema::projects::dsl as projects_dsl;
use crate::schema::repositories::dsl as repositories_dsl;

use crate::db::{
    errors::DBError,
    pool::{DBAccess, DBAccessor},
};
use crate::types::PaginationParams;
use crate::utils;
pub trait DBIssue: Send + Sync + Clone + 'static {
    fn all(
        &self,
        params: QueryParams,
        pagination: PaginationParams,
    ) -> Result<(Vec<Issue>, i64), DBError>;
    fn by_id(&self, id: i32) -> Result<Option<Issue>, DBError>;
    fn by_number(&self, repository_id: i32, number: i32) -> Result<Option<Issue>, DBError>;
    fn create(&self, issue: &NewIssue) -> Result<Issue, DBError>;
    fn update(&self, id: i32, issue: &UpdateIssue) -> Result<Issue, DBError>;
    fn delete_issue_assignee(&self, id: i32) -> Result<(), DBError>;
    fn delete(&self, id: i32) -> Result<(), DBError>;
}

impl DBIssue for DBAccess {
    fn all(
        &self,
        params: QueryParams,
        pagination: PaginationParams,
    ) -> Result<(Vec<Issue>, i64), DBError> {
        let conn = &mut self.get_db_conn();

        let build_query = || {
            let mut query = issues_dsl::issues
                .inner_join(
                    repositories_dsl::repositories
                        .on(issues_dsl::repository_id.eq(repositories_dsl::id)),
                )
                .inner_join(
                    projects_dsl::projects.on(repositories_dsl::project_id.eq(projects_dsl::id)),
                )
                .left_join(
                    languages_dsl::languages
                        .on(repositories_dsl::language_slug.eq(languages_dsl::slug)),
                )
                .into_boxed();

            if let Some(slug) = params.slug.as_ref() {
                query = query.filter(projects_dsl::slug.eq(slug));
            }

            if let Some(purpose) = params.purposes.as_ref() {
                query = query.filter(projects_dsl::purposes.contains(vec![purpose]));
            }

            if let Some(stack_level) = params.stack_levels.as_ref() {
                query = query.filter(projects_dsl::stack_levels.contains(vec![stack_level]));
            }

            if let Some(technology) = params.technologies.as_ref() {
                query = query.filter(projects_dsl::technologies.contains(vec![technology]));
            }

            if let Some(language_slug) = params.language_slug.as_ref() {
                query = query.filter(languages_dsl::slug.eq(language_slug));
            }

            if let Some(raw_labels) = params.labels.as_ref() {
                let labels: Vec<String> = utils::parse_comma_values(raw_labels);
                query = query.filter(issues_dsl::labels.overlaps_with(labels));
            }

            if let Some(open) = params.open.as_ref() {
                query = query.filter(issues_dsl::open.eq(open));
            }

            if let Some(assignee_id) = params.assignee_id.as_ref() {
                query = query.filter(issues_dsl::assignee_id.eq(assignee_id));
            }

            if let Some(repository_id) = params.repository_id.as_ref() {
                query = query.filter(issues_dsl::repository_id.eq(repository_id));
            }

            query
        };

        let total_count = build_query().count().get_result::<i64>(conn)?;

        let result = build_query()
            .offset(pagination.offset)
            .limit(pagination.limit)
            .select(issues_dsl::issues::all_columns())
            .load::<Issue>(conn)?;

        Ok((result, total_count))
    }
    fn by_id(&self, id: i32) -> Result<Option<Issue>, DBError> {
        let conn = &mut self.get_db_conn();
        let result = issues_dsl::issues
            .find(id)
            .first::<Issue>(conn)
            .optional()
            .map_err(DBError::from)?;
        Ok(result)
    }
    fn by_number(&self, repository_id: i32, number: i32) -> Result<Option<Issue>, DBError> {
        let conn = &mut self.get_db_conn();
        let result = issues_dsl::issues
            .filter(issues_dsl::repository_id.eq(repository_id))
            .filter(issues_dsl::number.eq(number))
            .first::<Issue>(conn)
            .optional()
            .map_err(DBError::from)?;
        Ok(result)
    }
    fn create(&self, form: &NewIssue) -> Result<Issue, DBError> {
        let conn = &mut self.get_db_conn();
        let issue = diesel::insert_into(issues_dsl::issues)
            .values(form)
            .get_result(conn)
            .map_err(DBError::from)?;

        Ok(issue)
    }

    fn update(&self, id: i32, issue: &UpdateIssue) -> Result<Issue, DBError> {
        let conn = &mut self.get_db_conn();

        let issue = diesel::update(issues_dsl::issues.filter(issues_dsl::id.eq(id)))
            .set((issue, issues_dsl::updated_at.eq(now)))
            .get_result::<Issue>(conn)
            .map_err(DBError::from)?;

        Ok(issue)
    }
    fn delete_issue_assignee(&self, id: i32) -> Result<(), DBError> {
        let conn = &mut self.get_db_conn();
        let query =
            format!("UPDATE issues SET assignee_id = NULL, updated_at = now() WHERE id = {id}");

        sql_query(query).execute(conn).map_err(DBError::from)?;

        Ok(())
    }

    fn delete(&self, id: i32) -> Result<(), DBError> {
        let conn = &mut self.get_db_conn();
        diesel::delete(issues_dsl::issues.filter(issues_dsl::id.eq(id)))
            .execute(conn)
            .map_err(DBError::from)?;

        Ok(())
    }
}
