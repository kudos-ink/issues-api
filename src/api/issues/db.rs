use diesel::dsl::now;
use diesel::prelude::*;
use diesel::sql_query;

use super::models::{Issue, IssueResponse, NewIssue, QueryParams, UpdateIssue};
use crate::api::projects::models::Project;
use crate::api::projects::models::ProjectResponse;
use crate::api::repositories::models::Repository;
use crate::api::repositories::models::RepositoryResponse;
use crate::schema::issues::dsl as issues_dsl;
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
    ) -> Result<(Vec<IssueResponse>, i64), DBError>;
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
    ) -> Result<(Vec<IssueResponse>, i64), DBError> {
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
                    users_dsl::users.on(issues_dsl::assignee_id.eq(users_dsl::id.nullable())),
                )
                .into_boxed();

            if let Some(slugs) = params.slugs.as_ref() {
                query = query.filter(projects_dsl::slug.eq_any(utils::parse_comma_values(slugs)));
            }
            if let Some(purposes) = params.purposes.as_ref() {
                query = query.filter(projects_dsl::purposes.overlaps_with( utils::parse_comma_values(purposes)));
            }
            if let Some(stack_levels) = params.stack_levels.as_ref() {
                    query = query.filter(projects_dsl::stack_levels.overlaps_with(utils::parse_comma_values(stack_levels)));
            }
            if let Some(technologies) = params.technologies.as_ref() {
                    query = query.filter(projects_dsl::technologies.overlaps_with(utils::parse_comma_values(technologies)));
            }
            if let Some(language_slug) = params.language_slugs.as_ref() {
                    query = query.filter(repositories_dsl::language_slug.eq_any(utils::parse_comma_values(language_slug)));
            }
            if let Some(labels) = params.labels.as_ref() {
                    query = query.filter(issues_dsl::labels.overlaps_with(utils::parse_comma_values(labels)));
            }

            if let Some(certified) = params.certified.as_ref() {
                query = query.filter(issues_dsl::certified.eq(certified));
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
            if let Some(rewards) = params.rewards.as_ref() {
                    query = query.filter(projects_dsl::rewards.eq(rewards));
            }
            query
        };

        let total_count = build_query().count().get_result::<i64>(conn)?;

        let result = build_query()
            .order(issues_dsl::issue_created_at.desc())
            .offset(pagination.offset)
            .limit(pagination.limit)
            .select((
                issues_dsl::issues::all_columns(),
                repositories_dsl::repositories::all_columns(),
                projects_dsl::projects::all_columns(),
                users_dsl::username.nullable(),
                users_dsl::avatar.nullable(),
            ))
            .load::<(Issue, Repository, Project, Option<String>, Option<String>)>(conn)?;

        let issues_full = result
            .into_iter()
            .map(|(issue, repo, project, username, avatar)| IssueResponse {
                id: issue.id,
                issue_id: issue.number,
                labels: issue.labels,
                open: issue.open,
                assignee_id: issue.assignee_id,
                assignee_username: username,
                assignee_avatar: avatar,
                title: issue.title,
                certified: issue.certified.unwrap_or(false),
                repository: RepositoryResponse {
                    id: repo.id,
                    slug: repo.slug,
                    name: repo.name,
                    url: repo.url,
                    language_slug: repo.language_slug,
                    project: ProjectResponse {
                        id: project.id,
                        name: project.name,
                        slug: project.slug,
                        purposes: project.purposes,
                        stack_levels: project.stack_levels,
                        technologies: project.technologies,
                        avatar: project.avatar,
                        created_at: project.created_at,
                        updated_at: project.updated_at,
                        rewards: project.rewards,
                    },
                    created_at: repo.created_at,
                    updated_at: repo.updated_at,
                },
                issue_created_at: issue.issue_created_at,
                issue_closed_at: issue.issue_closed_at,
                created_at: issue.created_at,
                updated_at: issue.updated_at,
                description: issue.description,
            })
            .collect();

        Ok((issues_full, total_count))
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
