use diesel::dsl::now;
use diesel::prelude::*;
use diesel::sql_query;

use super::models::IssueWithUsername;
use super::models::{Issue, NewIssue, QueryParams, UpdateIssue};
use crate::schema::issues::dsl as issues_dsl;
use crate::schema::languages::dsl as languages_dsl;
use crate::schema::projects::dsl as projects_dsl;
use crate::schema::repositories::dsl as repositories_dsl;
use crate::schema::users::dsl as users_dsl;

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
    ) -> Result<(Vec<IssueWithUsername>, i64), DBError>;
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
    ) -> Result<(Vec<IssueWithUsername>, i64), DBError> {
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
                .left_join(
                    users_dsl::users.on(issues_dsl::assignee_id.eq(users_dsl::id.nullable())),
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

            if let Some(has_assignee) = params.has_assignee.as_ref() {
                if *has_assignee {
                    query = query.filter(issues_dsl::assignee_id.is_not_null());
                } else {
                    query = query.filter(issues_dsl::assignee_id.is_null());
                }
            }

            if let Some(closed_at_min) = params.issue_closed_at_min.as_ref() {
                query = query.filter(
                    issues_dsl::issue_closed_at
                        .is_not_null()
                        .and(issues_dsl::issue_closed_at.ge(closed_at_min)),
                );
            }

            if let Some(closed_at_max) = params.issue_closed_at_max.as_ref() {
                query = query.filter(
                    issues_dsl::issue_closed_at
                        .is_not_null()
                        .and(issues_dsl::issue_closed_at.le(closed_at_max)),
                );
            }

            query
        };

        let total_count = build_query().count().get_result::<i64>(conn)?;

        let result = build_query()
            .offset(pagination.offset)
            .limit(pagination.limit)
            .select((
                issues_dsl::issues::all_columns(),
                users_dsl::username.nullable(),
            ))
            .load::<(Issue, Option<String>)>(conn)
            .map(|results| {
                results
                    .into_iter()
                    .map(|(issue, username)| IssueWithUsername {
                        id: issue.id,
                        number: issue.number,
                        title: issue.title,
                        labels: issue.labels,
                        open: issue.open,
                        certified: issue.certified,
                        assignee_id: issue.assignee_id,
                        assignee_username: username,
                        repository_id: issue.repository_id,
                        issue_created_at: issue.issue_created_at,
                        issue_closed_at: issue.issue_closed_at,
                        created_at: issue.created_at,
                        updated_at: issue.updated_at,
                    })
                    .collect::<Vec<IssueWithUsername>>()
            })?;

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
