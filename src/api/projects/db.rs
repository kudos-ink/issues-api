use std::collections::HashSet;

use diesel::dsl::now;
use diesel::prelude::*;

use super::models::{NewProject, Project, ProjectOptions, QueryParams, UpdateProject};
use crate::schema::issues::dsl as issues_dsl;
use crate::schema::projects::dsl as projects_dsl;
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
    fn options(&self, params: QueryParams) -> Result<ProjectOptions, DBError>;
    fn by_id(&self, id: i32) -> Result<Option<Project>, DBError>;
    fn by_slug(&self, slug: &str) -> Result<Option<Project>, DBError>;
    fn create(&self, form: &NewProject) -> Result<Project, DBError>;
    fn update(&self, id: i32, form: &UpdateProject) -> Result<Project, DBError>;
    fn delete(&self, id: i32) -> Result<(), DBError>;
}

impl DBProject for DBAccess {
    fn options(&self, params: QueryParams) -> Result<ProjectOptions, DBError> {
        let conn = &mut self.get_db_conn();
        println!("{:?}", params);
        let project_ids: Option<Vec<i32>> = {
            let base = issues_dsl::issues
                .inner_join(
                    repositories_dsl::repositories
                        .on(issues_dsl::repository_id.eq(repositories_dsl::id)),
                )
                .inner_join(
                    projects_dsl::projects.on(repositories_dsl::project_id.eq(projects_dsl::id)),
                )
                .select(projects_dsl::id);

                match (
                    params.certified_or_labels,
                    params.labels.as_ref(),
                    params.certified.as_ref(),
                    params.open,
                ) {
                    // Case 1: certified_or_labels is true, both labels and certified are provided
                    (Some(true), Some(labels), Some(certified), Some(open)) => base
                        .filter(
                            issues_dsl::labels
                                .overlaps_with(utils::parse_comma_values(labels))
                                .or(issues_dsl::certified.eq(certified))
                                .and(issues_dsl::open.eq(open)),
                        )
                        .distinct()
                        .load::<i32>(conn)
                        .optional()?,
                    
                    // Case 2: certified_or_labels is true, only labels are provided
                    (Some(true), Some(labels), None, Some(open)) => base
                        .filter(
                            issues_dsl::labels
                                .overlaps_with(utils::parse_comma_values(labels))
                                .and(issues_dsl::open.eq(open)),
                        )
                        .distinct()
                        .load::<i32>(conn)
                        .optional()?,
                    
                    // Case 3: certified_or_labels is true, only certified is provided
                    (Some(true), None, Some(certified), Some(open)) => base
                        .filter(
                            issues_dsl::certified
                                .eq(certified)
                                .and(issues_dsl::open.eq(open)),
                        )
                        .distinct()
                        .load::<i32>(conn)
                        .optional()?,
                    
                    // Case 4: certified_or_labels is true, neither labels nor certified are provided, but open is specified
                    (Some(true), None, None, Some(open)) => base
                        .filter(issues_dsl::open.eq(open))
                        .distinct()
                        .load::<i32>(conn)
                        .optional()?,
                    
                    // Case 5: certified_or_labels is false or None, both labels and certified are provided
                    (_, Some(labels), Some(certified), Some(open)) => base
                        .filter(
                            issues_dsl::labels
                                .overlaps_with(utils::parse_comma_values(labels))
                                .and(issues_dsl::certified.eq(certified))
                                .and(issues_dsl::open.eq(open)),
                        )
                        .distinct()
                        .load::<i32>(conn)
                        .optional()?,
                    
                    // Case 6: certified_or_labels is false or None, only labels are provided
                    (_, Some(labels), None, Some(open)) => base
                        .filter(
                            issues_dsl::labels
                                .overlaps_with(utils::parse_comma_values(labels))
                                .and(issues_dsl::open.eq(open)),
                        )
                        .distinct()
                        .load::<i32>(conn)
                        .optional()?,
                    
                    // Case 7: certified_or_labels is false or None, only certified is provided
                    (_, None, Some(certified), Some(open)) => base
                        .filter(
                            issues_dsl::certified
                                .eq(certified)
                                .and(issues_dsl::open.eq(open)),
                        )
                        .distinct()
                        .load::<i32>(conn)
                        .optional()?,
                    
                    // Case 8: certified_or_labels is false or None, and open is specified, no labels or certified
                    (_, None, None, Some(open)) => issues_dsl::issues
                        .inner_join(
                            repositories_dsl::repositories
                                .on(issues_dsl::repository_id.eq(repositories_dsl::id)),
                        )
                        .inner_join(
                            projects_dsl::projects
                                .on(repositories_dsl::project_id.eq(projects_dsl::id)),
                        )
                        .select(projects_dsl::id)
                        .filter(issues_dsl::open.eq(open))
                        .distinct()
                        .load::<i32>(conn)
                        .optional()?,
                    
                    // Case 9: certified_or_labels is true, only labels are provided and open is not specified
                    (Some(true), Some(labels), None, None) => base
                        .filter(
                            issues_dsl::labels
                                .overlaps_with(utils::parse_comma_values(labels)),
                        )
                        .distinct()
                        .load::<i32>(conn)
                        .optional()?,
                    
                    // Case 10: certified_or_labels is true, only certified is provided and open is not specified
                    (Some(true), None, Some(certified), None) => base
                        .filter(
                            issues_dsl::certified.eq(certified),
                        )
                        .distinct()
                        .load::<i32>(conn)
                        .optional()?,
                    
                    // Case 11: certified_or_labels is false or None, both labels and certified are provided but open is not specified
                    (_, Some(labels), Some(certified), None) => base
                        .filter(
                            issues_dsl::labels
                                .overlaps_with(utils::parse_comma_values(labels))
                                .and(issues_dsl::certified.eq(certified)),
                        )
                        .distinct()
                        .load::<i32>(conn)
                        .optional()?,
                    
                    // Case 12: certified_or_labels is false or None, only labels are provided and open is not specified
                    (_, Some(labels), None, None) => base
                        .filter(
                            issues_dsl::labels
                                .overlaps_with(utils::parse_comma_values(labels)),
                        )
                        .distinct()
                        .load::<i32>(conn)
                        .optional()?,
                    
                    // Case 13: certified_or_labels is false or None, only certified is provided and open is not specified
                    (_, None, Some(certified), None) => base
                        .filter(
                            issues_dsl::certified.eq(certified),
                        )
                        .distinct()
                        .load::<i32>(conn)
                        .optional()?,
                    
                    // Case 14: certified_or_labels is false or None, no labels or certified, and open is not specified
                    (_, None, None, None) => base
                        .distinct()
                        .load::<i32>(conn)
                        .optional()?,
                }
        };

        let build_query = || {
            let mut query = projects_dsl::projects.into_boxed();
            if let Some(slugs) = params.slugs.as_ref() {
                query = query.filter(projects_dsl::slug.eq_any(utils::parse_comma_values(slugs)));
            }
            if let Some(purposes) = params.purposes.as_ref() {
                query = query.filter(
                    projects_dsl::purposes.overlaps_with(utils::parse_comma_values(purposes)),
                );
            }
            if let Some(technologies) = params.technologies.as_ref() {
                query = query.filter(
                    projects_dsl::technologies
                        .overlaps_with(utils::parse_comma_values(technologies)),
                );
            }
            if let Some(stack_levels) = params.stack_levels.as_ref() {
                query = query.filter(
                    projects_dsl::stack_levels
                        .overlaps_with(utils::parse_comma_values(stack_levels)),
                );
            }
            if let Some(rewards) = params.rewards.as_ref() {
                query = query.filter(projects_dsl::rewards.eq(rewards));
            }

            if let Some(project_ids) = project_ids.as_ref() {
                query = query.filter(projects_dsl::id.eq_any(project_ids));
            }

            query
        };

        let result = build_query().load::<Project>(conn)?;

        let mut unique_types = HashSet::new();
        let mut unique_purposes = HashSet::new();
        let mut unique_technologies = HashSet::new();
        let mut unique_stack_levels = HashSet::new();

        for project in &result {
            if let Some(types) = &project.types {
                for type_option in types {
                    unique_types.insert(type_option.clone());
                }
            }

            if let Some(purposes) = &project.purposes {
                for purpose_option in purposes {
                    unique_purposes.insert(purpose_option.clone());
                }
            }
            if let Some(stack_levels) = &project.stack_levels {
                for stack_level in stack_levels {
                    unique_stack_levels.insert(stack_level.clone());
                }
            }
            if let Some(technologies) = &project.technologies {
                for technology in technologies {
                    unique_technologies.insert(technology.clone());
                }
            }
        }

        let project_options = ProjectOptions {
            types: Some(unique_types.into_iter().collect()),
            purposes: Some(unique_purposes.into_iter().collect()),
            technologies: Some(unique_technologies.into_iter().collect()),
            stack_levels: Some(unique_stack_levels.into_iter().collect()),
        };

        Ok(project_options)
    }
    fn all(
        &self,
        params: QueryParams,
        pagination: PaginationParams,
    ) -> Result<(Vec<Project>, i64), DBError> {
        let conn = &mut self.get_db_conn();

        // filter by labels
        let project_ids: Option<Vec<i32>> = if let Some(certified) = params.certified.as_ref() {
            issues_dsl::issues
                .inner_join(
                    repositories_dsl::repositories
                        .on(issues_dsl::repository_id.eq(repositories_dsl::id)),
                )
                .inner_join(
                    projects_dsl::projects.on(repositories_dsl::project_id.eq(projects_dsl::id)),
                )
                .select(projects_dsl::id)
                .filter(issues_dsl::certified.eq(certified))
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
                query = query.filter(
                    projects_dsl::purposes.overlaps_with(utils::parse_comma_values(purposes)),
                );
            }
            if let Some(technologies) = params.technologies.as_ref() {
                query = query.filter(
                    projects_dsl::technologies
                        .overlaps_with(utils::parse_comma_values(technologies)),
                );
            }
            if let Some(stack_levels) = params.stack_levels.as_ref() {
                query = query.filter(
                    projects_dsl::stack_levels
                        .overlaps_with(utils::parse_comma_values(stack_levels)),
                );
            }
            if let Some(rewards) = params.rewards.as_ref() {
                query = query.filter(projects_dsl::rewards.eq(rewards));
            }

            if let Some(project_ids) = project_ids.as_ref() {
                query = query.filter(projects_dsl::id.eq_any(project_ids));
            }
            if let Some(types) = params.types.as_ref() {
                query = query.filter(
                    projects_dsl::types
                        .overlaps_with(utils::parse_comma_values(types)),
                );
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
