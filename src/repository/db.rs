use mobc::async_trait;
use mobc_postgres::tokio_postgres::Row;
use warp::reject;

use crate::db::{
    pool::DBAccess,
    utils::{
        execute_query_with_timeout, query_one_timeout, query_opt_timeout, query_with_timeout,
        DB_QUERY_TIMEOUT,
    },
};

use super::models::{Repository, RepositoryCreateRequest};

const TABLE: &str = "repositories";

#[async_trait]
pub trait DBRepository: Send + Sync + Clone + 'static {
    async fn get_repository(&self, id: i32) -> Result<Option<Repository>, reject::Rejection>;
    async fn get_repository_by_name(
        &self,
        name: &str,
    ) -> Result<Option<Repository>, reject::Rejection>;
    async fn get_repositories(&self) -> Result<Vec<Repository>, reject::Rejection>;
    async fn create_repository(
        &self,
        repository: RepositoryCreateRequest,
    ) -> Result<Repository, reject::Rejection>;
    async fn delete_repository(&self, id: i32) -> Result<(), reject::Rejection>;
}

#[async_trait]
impl DBRepository for DBAccess {
    async fn get_repository(&self, id: i32) -> Result<Option<Repository>, reject::Rejection> {
        let query = format!("SELECT * FROM {} WHERE id = $1", TABLE);
        match query_opt_timeout(self, query.as_str(), &[&id], DB_QUERY_TIMEOUT).await? {
            Some(repository) => Ok(Some(row_to_repository(&repository))),
            None => Ok(None),
        }
    }
    async fn get_repository_by_name(
        &self,
        name: &str,
    ) -> Result<Option<Repository>, reject::Rejection> {
        let query = format!("SELECT * FROM {} WHERE name = $1", TABLE);
        match query_opt_timeout(self, query.as_str(), &[&name], DB_QUERY_TIMEOUT).await? {
            Some(repository) => Ok(Some(row_to_repository(&repository))),
            None => Ok(None),
        }
    }

    async fn get_repositories(&self) -> Result<Vec<Repository>, reject::Rejection> {
        let query = format!("SELECT * FROM {} ORDER BY created_at DESC", TABLE);
        let rows = query_with_timeout(self, query.as_str(), &[], DB_QUERY_TIMEOUT).await?;
        Ok(rows.iter().map(row_to_repository).collect())
    }

    async fn create_repository(
        &self,
        repository: RepositoryCreateRequest,
    ) -> Result<Repository, reject::Rejection> {
        let query = format!(
            "INSERT INTO {} (name, organization_id) VALUES ($1, $2) RETURNING *",
            TABLE
        );
        let row = query_one_timeout(
            self,
            &query,
            &[&repository.name, &repository.organization_id],
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
    Repository {
        id,
        name: name.to_string(),
        organization_id,
    }
}
