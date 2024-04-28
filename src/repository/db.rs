use mobc::async_trait;
use mobc_postgres::tokio_postgres::Row;
use warp::reject;

use super::models::{NewRepository, RepositoriesRelations, Repository, RepositorySort};
use crate::db::{
    pool::DBAccess,
    utils::{
        execute_query_with_timeout, query_one_timeout, query_opt_timeout, query_with_timeout,
        DB_QUERY_TIMEOUT,
    },
};
use crate::pagination::GetPagination;

const TABLE: &str = "repositories";

#[async_trait]
pub trait DBRepository: Send + Sync + Clone + 'static {
    async fn get_repository(
        &self,
        id: i32,
        relations: RepositoriesRelations,
    ) -> Result<Option<Repository>, reject::Rejection>;
    async fn get_repository_by_name(
        &self,
        name: &str,
        relations: RepositoriesRelations,
    ) -> Result<Option<Repository>, reject::Rejection>;
    async fn get_repositories(
        &self,
        relations: RepositoriesRelations,
        pagination: GetPagination,
        sort: RepositorySort,
    ) -> Result<Vec<Repository>, reject::Rejection>;
    async fn create_repository(
        &self,
        repository: NewRepository,
    ) -> Result<Repository, reject::Rejection>;
    async fn delete_repository(&self, id: i32) -> Result<(), reject::Rejection>;
}

#[async_trait]
impl DBRepository for DBAccess {
    async fn get_repository(
        &self,
        id: i32,
        relations: RepositoriesRelations,
    ) -> Result<Option<Repository>, reject::Rejection> {
        let mut query = format!("SELECT * FROM {} ", TABLE);
        if relations.issues {
            query += "LEFT JOIN issues on issues.repository_id = repositories.id ";
            if relations.tips {
                query += "LEFT JOIN tips on tips.id = issues.id ";
            }
        }
        if relations.maintainers {
            query += "LEFT JOIN maintainers on maintainers.repository_id = repositories.id ";
            query += "LEFT JOIN users on maintainers.user_id = users.id ";
        }
        if relations.languages {
            query += "LEFT JOIN languages on repositories.languages_id = languages.id ";
        }
        query += "WHERE id = $1";

        match query_opt_timeout(self, query.as_str(), &[&id], DB_QUERY_TIMEOUT).await? {
            Some(repository) => Ok(Some(row_to_repository(&repository))),
            None => Ok(None),
        }
    }
    async fn get_repository_by_name(
        &self,
        name: &str,
        relations: RepositoriesRelations,
    ) -> Result<Option<Repository>, reject::Rejection> {
        let mut query = format!("SELECT * FROM {} ", TABLE);
        if relations.issues {
            query += "LEFT JOIN issues on issues.repository_id = repositories.id ";
            if relations.tips {
                query += "LEFT JOIN tips on tips.id = issues.id ";
            }
        }
        if relations.maintainers {
            query += "LEFT JOIN maintainers on maintainers.repository_id = repositories.id ";
            query += "LEFT JOIN users on maintainers.user_id = users.id ";
        }
        if relations.languages {
            query += "LEFT JOIN languages on repositories.languages_id = languages.id ";
        }
        query += "WHERE name = $1";
        match query_opt_timeout(self, query.as_str(), &[&name], DB_QUERY_TIMEOUT).await? {
            Some(repository) => Ok(Some(row_to_repository(&repository))),
            None => Ok(None),
        }
    }

    async fn get_repositories(
        &self,
        relations: RepositoriesRelations,
        pagination: GetPagination,
        sort: RepositorySort,
    ) -> Result<Vec<Repository>, reject::Rejection> {
        let mut query = format!("SELECT * FROM {} ", TABLE);
        if relations.issues {
            query += "LEFT JOIN issues on issues.repository_id = repositories.id ";
            if relations.tips {
                query += "LEFT JOIN tips on tips.id = issues.id ";
            }
        }
        if relations.maintainers {
            query += "LEFT JOIN maintainers on maintainers.repository_id = repositories.id ";
            query += "LEFT JOIN users on maintainers.user_id = users.id ";
        }
        if relations.languages {
            query += "LEFT JOIN languages on repositories.languages_id = languages.id ";
        }
        query += &format!("ORDER BY {} {}", sort.field, sort.order); // cannot use $1 or $2

        query += "LIMIT $1 OFFSET $2";
        let rows = query_with_timeout(
            self,
            query.as_str(),
            &[&pagination.limit, &pagination.offset],
            DB_QUERY_TIMEOUT,
        )
        .await?;
        Ok(rows.iter().map(row_to_repository).collect())
    }

    async fn create_repository(
        &self,
        repository: NewRepository,
    ) -> Result<Repository, reject::Rejection> {
        let query = format!(
            "INSERT INTO {} (name, icon, organization_id, url, e_tag) VALUES ($1, $2, $3, $4, $5) RETURNING *",
            TABLE
        );
        let row = query_one_timeout(
            self,
            &query,
            &[
                &repository.name,
                &repository.icon,
                &repository.organization_id,
                &repository.url,
                &repository.e_tag,
            ],
            DB_QUERY_TIMEOUT,
        )
        .await?;
        Ok(row_to_repository(&row))
    }

    async fn delete_repository(&self, id: i32) -> Result<(), reject::Rejection> {
        let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
        execute_query_with_timeout(self, &query, &[&id], DB_QUERY_TIMEOUT).await
    }
}

fn row_to_repository(row: &Row) -> Repository {
    let id: i32 = row.get(0);
    let name: &str = row.get(1);
    let organization_id: i32 = row.get(2);
    let icon: &str = row.get(3);
    let url: &str = row.get(4);
    let e_tag: &str = row.get(5);
    Repository {
        id,
        name: name.to_string(),
        organization_id,
        icon: icon.to_string(),
        url: url.to_string(),
        e_tag: e_tag.to_string(),
    }
}
