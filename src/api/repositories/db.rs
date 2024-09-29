use diesel::dsl::sql;
use diesel::sql_types::Text;
use diesel::{dsl::now, prelude::*};

use super::models::{NewRepository, QueryParams, Repository, UpdateRepository};
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
    fn aggregate_languages(&self) -> Result<Vec<String>, DBError>;
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
            query = query.filter(repositories_dsl::language_slug.eq_any(utils::parse_comma_values(&languages)));
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

    fn aggregate_languages(&self) -> Result<Vec<String>, DBError> {
        let conn = &mut self.get_db_conn();
        let languages = repositories_dsl::repositories
            .select(sql::<Text>("DISTINCT language_slug"))
            .load::<String>(conn)?;
        Ok(languages)
    }
}
