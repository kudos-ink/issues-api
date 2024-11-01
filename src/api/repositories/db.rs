use std::collections::HashSet;

use diesel::dsl::sql;
use diesel::sql_types::Text;
use diesel::{dsl::now, prelude::*};

use super::models::{
    LanguageQueryParams, NewRepository, QueryParams, Repository, UpdateRepository,
};
use crate::schema::issues::dsl as issues_dsl;
use crate::schema::projects::dsl as projects_dsl;
use crate::schema::repositories::dsl as repositories_dsl;
use crate::utils;
use crate::{
    db::{
        errors::DBError,
        pool::{DBAccess, DBAccessor},
    },
    types::PaginationParams,
};

pub trait DBRepository: Send + Sync + Clone + 'static {
    fn by_id(&self, id: i32) -> Result<Option<Repository>, DBError>;
    fn all(
        &self,
        params: QueryParams,
        pagination: PaginationParams,
    ) -> Result<Vec<Repository>, DBError>;
    fn create(&self, repo: &NewRepository) -> Result<Repository, DBError>;
    fn update(&self, id: i32, repo: &UpdateRepository) -> Result<Repository, DBError>;
    fn delete(&self, id: i32) -> Result<(), DBError>;
    fn by_slug(&self, slug: &str) -> Result<Option<Repository>, DBError>;
    fn aggregate_languages(&self, params: LanguageQueryParams) -> Result<Vec<String>, DBError>;
}

impl DBRepository for DBAccess {
    fn all(
        &self,
        params: QueryParams,
        pagination: PaginationParams,
    ) -> Result<Vec<Repository>, DBError> {
        let conn = &mut self.get_db_conn();
        let mut query = repositories_dsl::repositories.into_boxed();

        if let Some(languages) = params.languages {
            query = query.filter(
                repositories_dsl::language_slug.eq_any(utils::parse_comma_values(&languages)),
            );
        }
        if let Some(slugs) = params.slugs {
            query = query.filter(repositories_dsl::slug.eq_any(utils::parse_comma_values(&slugs)));
        }
        if let Some(names) = params.names {
            query = query.filter(repositories_dsl::name.eq_any(utils::parse_comma_values(&names)));
        }

        if let Some(project_id) = params.project_ids {
            let ids: Vec<i32> = utils::parse_ids(&project_id);
            if !ids.is_empty() {
                query = query.filter(repositories_dsl::project_id.eq_any(ids));
            }
        }

        query = query.offset(pagination.offset).limit(pagination.limit);

        let result = query.load::<Repository>(conn)?;
        Ok(result)
    }

    fn by_id(&self, id: i32) -> Result<Option<Repository>, DBError> {
        let conn = &mut self.get_db_conn();

        let result = repositories_dsl::repositories
            .find(id)
            .first::<Repository>(conn)
            .optional()
            .map_err(DBError::from)?;

        Ok(result)
    }

    fn create(&self, repository: &NewRepository) -> Result<Repository, DBError> {
        let conn = &mut self.get_db_conn();

        let repository = diesel::insert_into(repositories_dsl::repositories)
            .values(repository)
            .get_result(conn)
            .map_err(DBError::from)?;

        Ok(repository)
    }

    fn update(&self, id: i32, repository: &UpdateRepository) -> Result<Repository, DBError> {
        let conn = &mut self.get_db_conn();

        let project =
            diesel::update(repositories_dsl::repositories.filter(repositories_dsl::id.eq(id)))
                .set((repository, repositories_dsl::updated_at.eq(now)))
                .get_result::<Repository>(conn)
                .map_err(DBError::from)?;

        Ok(project)
    }

    fn delete(&self, id: i32) -> Result<(), DBError> {
        let conn = &mut self.get_db_conn();
        diesel::delete(repositories_dsl::repositories.filter(repositories_dsl::id.eq(id)))
            .execute(conn)
            .map_err(DBError::from)?;

        Ok(())
    }

    fn by_slug(&self, slug: &str) -> Result<Option<Repository>, DBError> {
        let conn = &mut self.get_db_conn();

        let result = repositories_dsl::repositories
            .filter(repositories_dsl::slug.eq(slug))
            .first::<Repository>(conn)
            .optional()
            .map_err(DBError::from)?;

        Ok(result)
    }

    fn aggregate_languages(&self, params: LanguageQueryParams) -> Result<Vec<String>, DBError> {
        let conn = &mut self.get_db_conn();

        let mut query = repositories_dsl::repositories
            .inner_join(issues_dsl::issues.on(issues_dsl::repository_id.eq(repositories_dsl::id)))
            .inner_join(
                projects_dsl::projects.on(repositories_dsl::project_id.eq(projects_dsl::id)),
            )
            .select(sql::<Text>("DISTINCT language_slug"))
            .filter(repositories_dsl::language_slug.is_not_null())
            .into_boxed();

        if let Some(slugs) = params.slugs.as_ref() {
            query = query.filter(projects_dsl::slug.eq_any(utils::parse_comma_values(slugs)));
        }

        if let Some(true) = params.certified_or_labels {
            if let (Some(labels), Some(certified)) =
                (params.labels.as_ref(), params.certified.as_ref())
            {
                query = query.filter(
                    issues_dsl::labels
                        .overlaps_with(utils::parse_comma_values(labels))
                        .or(issues_dsl::certified.eq(certified)),
                );
            } else if let Some(labels) = params.labels.as_ref() {
                query = query
                    .filter(issues_dsl::labels.overlaps_with(utils::parse_comma_values(labels)));
            } else if let Some(certified) = params.certified.as_ref() {
                query = query.filter(issues_dsl::certified.eq(certified));
            }
        } else {
            if let Some(labels) = params.labels.as_ref() {
                query = query
                    .filter(issues_dsl::labels.overlaps_with(utils::parse_comma_values(labels)));
            }
            if let Some(certified) = params.certified.as_ref() {
                query = query.filter(issues_dsl::certified.eq(certified));
            }
        }

        if let Some(open) = params.open.as_ref() {
            query = query.filter(issues_dsl::open.eq(open));
        }

        let languages: HashSet<String> = query.load::<String>(conn)?.into_iter().collect();

        let mut technologies: HashSet<String> = HashSet::new();

        // Query for technologies if with_technologies is true
        if params.with_technologies.unwrap_or(false) {
            let mut tech_query = projects_dsl::projects
                .inner_join(
                    repositories_dsl::repositories
                        .on(repositories_dsl::project_id.eq(projects_dsl::id)),
                )
                .inner_join(
                    issues_dsl::issues.on(issues_dsl::repository_id.eq(repositories_dsl::id)),
                )
                .select(projects_dsl::technologies)
                .filter(projects_dsl::technologies.is_not_null())
                .into_boxed();

            if let Some(slugs) = params.slugs.as_ref() {
                tech_query =
                    tech_query.filter(projects_dsl::slug.eq_any(utils::parse_comma_values(slugs)));
            }

            if let Some(true) = params.certified_or_labels {
                if let (Some(labels), Some(certified)) =
                    (params.labels.as_ref(), params.certified.as_ref())
                {
                    tech_query = tech_query.filter(
                        issues_dsl::labels
                            .overlaps_with(utils::parse_comma_values(labels))
                            .or(issues_dsl::certified.eq(certified)),
                    );
                } else if let Some(labels) = params.labels.as_ref() {
                    tech_query = tech_query.filter(
                        issues_dsl::labels.overlaps_with(utils::parse_comma_values(labels)),
                    );
                } else if let Some(certified) = params.certified.as_ref() {
                    tech_query = tech_query.filter(issues_dsl::certified.eq(certified));
                }
            } else {
                if let Some(labels) = params.labels.as_ref() {
                    tech_query = tech_query.filter(
                        issues_dsl::labels.overlaps_with(utils::parse_comma_values(labels)),
                    );
                }
                if let Some(certified) = params.certified.as_ref() {
                    tech_query = tech_query.filter(issues_dsl::certified.eq(certified));
                }
            }

            if let Some(open) = params.open.as_ref() {
                tech_query = tech_query.filter(issues_dsl::open.eq(open));
            }

            let tech_results: Vec<Option<Vec<Option<String>>>> = tech_query.load(conn)?;

            // Collect technologies while flattening the nested structure
            for tech_list in tech_results {
                if let Some(list) = tech_list {
                    for tech in list {
                        if let Some(tech_name) = tech {
                            technologies.insert(tech_name);
                        }
                    }
                }
            }
        }

        let mut unique_items: HashSet<String> = languages;
        unique_items.extend(technologies);

        Ok(unique_items.into_iter().collect())
    }
}
