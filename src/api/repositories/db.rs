use diesel::{associations::HasTable, prelude::*};
use warp::reject;

use super::models::{NewRepository, RepositoriesRelations, Repository, RepositoryQueryParams};
use crate::schema::languages::dsl::*;
use crate::schema::repositories::dsl as repositories_dsl;
use crate::schema::repositories::dsl::*;
use crate::utils;
use crate::{
    db::{
        errors::DBError,
        pool::{DBAccess, DBAccessor},
    },
    languages::models::Language,
};
// use crate::pagination::GetPagination;

const TABLE: &str = "repositories";

pub trait DBRepository: Send + Sync + Clone + 'static {
    fn get_repository(&self, id: i32) -> Result<Option<Repository>, reject::Rejection>;
    fn get_repositories(
        &self,
        params: RepositoryQueryParams,
    ) -> Result<Vec<Repository>, reject::Rejection>;
    // async fn get_repository_by_name(
    //     &self,
    //     name: &str,
    //     relations: RepositoriesRelations,
    // ) -> Result<Option<Repository>, reject::Rejection>;
    // async fn get_repositories(
    //     &self,
    //     relations: RepositoriesRelations,
    //     pagination: GetPagination,
    //     sort: RepositorySort,
    // ) -> Result<Vec<Repository>, reject::Rejection>;
    // async fn create_repository(
    //     &self,
    //     repository: NewRepository,
    // ) -> Result<Repository, reject::Rejection>;
    // async fn delete_repository(&self, id: i32) -> Result<(), reject::Rejection>;
}

impl DBRepository for DBAccess {
    // async fn get_repository(
    //     &self,
    //     id: i32,
    //     relations: RepositoriesRelations,
    // ) -> Result<Option<Repository>, reject::Rejection> {
    //     let mut query = format!("SELECT * FROM {} ", TABLE);
    //     if relations.issues {
    //         query += "LEFT JOIN issues on issues.repository_id = repositories.id ";
    //         if relations.tips {
    //             query += "LEFT JOIN tips on tips.id = issues.id ";
    //         }
    //     }
    //     if relations.maintainers {
    //         query += "LEFT JOIN maintainers on maintainers.repository_id = repositories.id ";
    //         query += "LEFT JOIN users on maintainers.user_id = users.id ";
    //     }
    //     if relations.languages {
    //         query += "LEFT JOIN languages on repositories.languages_id = languages.id ";
    //     }
    //     query += "WHERE id = $1";

    //     match query_opt_timeout(self, query.as_str(), &[&id], DB_QUERY_TIMEOUT).await? {
    //         Some(repository) => Ok(Some(row_to_repository(&repository))),
    //         None => Ok(None),
    //     }
    // }

    fn get_repositories(
        &self,
        params: RepositoryQueryParams,
    ) -> Result<Vec<Repository>, reject::Rejection> {
        let conn = &mut self.get_db_conn();
        let mut query = repositories_dsl::repositories.into_boxed();

        if let Some(language_ids) = params.language {
            let ids: Vec<i32> = utils::parse_ids(&language_ids); // TODO: handle parsing error
            query = query.filter(repositories_dsl::language_id.eq_any(ids));
        }

        query
            .load::<Repository>(conn)
            .map_err(|e| reject::custom(DBError::DBQuery(e)))
    }

    fn get_repository(&self, repo_id: i32) -> Result<Option<Repository>, reject::Rejection> {
        let conn = &mut self.get_db_conn();
        repositories
            .find(repo_id)
            .first::<Repository>(conn)
            .optional()
            .map_err(|e| reject::custom(DBError::DBQuery(e)))
    }

    // async fn get_repository_by_name(
    //     &self,
    //     name: &str,
    //     relations: RepositoriesRelations,
    // ) -> Result<Option<Repository>, reject::Rejection> {
    //     let mut query = format!("SELECT * FROM {} ", TABLE);
    //     if relations.issues {
    //         query += "LEFT JOIN issues on issues.repository_id = repositories.id ";
    //         if relations.tips {
    //             query += "LEFT JOIN tips on tips.id = issues.id ";
    //         }
    //     }
    //     if relations.maintainers {
    //         query += "LEFT JOIN maintainers on maintainers.repository_id = repositories.id ";
    //         query += "LEFT JOIN users on maintainers.user_id = users.id ";
    //     }
    //     if relations.languages {
    //         query += "LEFT JOIN languages on repositories.languages_id = languages.id ";
    //     }
    //     query += "WHERE name = $1";
    //     match query_opt_timeout(self, query.as_str(), &[&name], DB_QUERY_TIMEOUT).await? {
    //         Some(repository) => Ok(Some(row_to_repository(&repository))),
    //         None => Ok(None),
    //     }
    // }

    // async fn get_repositories(
    //     &self,
    //     relations: RepositoriesRelations,
    //     pagination: GetPagination,
    //     sort: RepositorySort,
    // ) -> Result<Vec<Repository>, reject::Rejection> {
    //     let mut query = format!("SELECT * FROM {} ", TABLE);
    //     if relations.issues {
    //         query += "LEFT JOIN issues on issues.repository_id = repositories.id ";
    //         if relations.tips {
    //             query += "LEFT JOIN tips on tips.id = issues.id ";
    //         }
    //     }
    //     if relations.maintainers {
    //         query += "LEFT JOIN maintainers on maintainers.repository_id = repositories.id ";
    //         query += "LEFT JOIN users on maintainers.user_id = users.id ";
    //     }
    //     if relations.languages {
    //         query += "LEFT JOIN languages on repositories.languages_id = languages.id ";
    //     }
    //     query += &format!("ORDER BY {} {}", sort.field, sort.order); // cannot use $1 or $2

    //     query += "LIMIT $1 OFFSET $2";
    //     let rows = query_with_timeout(
    //         self,
    //         query.as_str(),
    //         &[&pagination.limit, &pagination.offset],
    //         DB_QUERY_TIMEOUT,
    //     )
    //     .await?;
    //     Ok(rows.iter().map(row_to_repository).collect())
    // }

    // async fn create_repository(
    //     &self,
    //     repository: NewRepository,
    // ) -> Result<Repository, reject::Rejection> {
    //     let query = format!(
    //         "INSERT INTO {} (name, icon, organization_id, url, e_tag) VALUES ($1, $2, $3, $4, $5) RETURNING *",
    //         TABLE
    //     );
    //     let row = query_one_timeout(
    //         self,
    //         &query,
    //         &[
    //             &repository.name,
    //             &repository.icon,
    //             &repository.organization_id,
    //             &repository.url,
    //             &repository.e_tag,
    //         ],
    //         DB_QUERY_TIMEOUT,
    //     )
    //     .await?;
    //     Ok(row_to_repository(&row))
    // }

    // async fn delete_repository(&self, id: i32) -> Result<(), reject::Rejection> {
    //     let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    //     execute_query_with_timeout(self, &query, &[&id], DB_QUERY_TIMEOUT).await
    // }
}
