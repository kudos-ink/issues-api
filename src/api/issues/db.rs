use diesel::dsl::now;
use diesel::prelude::*;

use super::models::{Issue, NewIssue, QueryParams, UpdateIssue};
use crate::schema::issues::dsl as issues_dsl;

use crate::db::{
    errors::DBError,
    pool::{DBAccess, DBAccessor},
};
use crate::types::PaginationParams;
use crate::utils;
pub trait DBIssue: Send + Sync + Clone + 'static {
    fn all(&self, params: QueryParams, pagination: PaginationParams)
        -> Result<Vec<Issue>, DBError>;
    fn by_id(&self, id: i32) -> Result<Option<Issue>, DBError>;
    fn by_number(&self, repository_id: i32, number: i32) -> Result<Option<Issue>, DBError>;
    fn create(&self, issue: &NewIssue) -> Result<Issue, DBError>;
    fn update(&self, id: i32, issue: &UpdateIssue) -> Result<Issue, DBError>;
    fn delete(&self, id: i32) -> Result<(), DBError>;
}

impl DBIssue for DBAccess {
    fn all(
        &self,
        params: QueryParams,
        pagination: PaginationParams,
    ) -> Result<Vec<Issue>, DBError> {
        let conn = &mut self.get_db_conn();
        let mut query = issues_dsl::issues.into_boxed();

        if let Some(raw_labels) = params.labels {
            let labels: Vec<String> = utils::parse_comma_values(&raw_labels);
            query = query.filter(issues_dsl::labels.overlaps_with(labels));
        }

        if let Some(open) = params.open {
            query = query.filter(issues_dsl::open.eq(open));
        }

        if let Some(assignee_id) = params.assignee_id {
            query = query.filter(issues_dsl::assignee_id.eq(assignee_id));
        }

        if let Some(repository_id) = params.repository_id {
            query = query.filter(issues_dsl::repository_id.eq(repository_id));
        }

        query = query.offset(pagination.offset).limit(pagination.limit);

        let result = query.load::<Issue>(conn)?;
        Ok(result)
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
        let project = diesel::insert_into(issues_dsl::issues)
            .values(form)
            .get_result(conn)
            .map_err(DBError::from)?;

        Ok(project)
    }

    fn update(&self, id: i32, issue: &UpdateIssue) -> Result<Issue, DBError> {
        let conn = &mut self.get_db_conn();

        let project = diesel::update(issues_dsl::issues.filter(issues_dsl::id.eq(id)))
            .set((issue, issues_dsl::updated_at.eq(now)))
            .get_result::<Issue>(conn)
            .map_err(DBError::from)?;

        Ok(project)
    }

    fn delete(&self, id: i32) -> Result<(), DBError> {
        let conn = &mut self.get_db_conn();
        diesel::delete(issues_dsl::issues.filter(issues_dsl::id.eq(id)))
            .execute(conn)
            .map_err(DBError::from)?;

        Ok(())
    }
}
