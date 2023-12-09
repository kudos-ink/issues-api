
use mobc::async_trait;
use mobc_postgres::tokio_postgres::Row;
use warp::reject;

use crate::db::{
    pool::DBAccess,
    utils::{ execute_query_with_timeout, query_opt_timeout, query_one_timeout, query_with_timeout, DB_QUERY_TIMEOUT},
};

use super::models::{Contribution, ContributionRequest};

const TABLE: &str = "contribution";

#[async_trait]
pub trait DBContribution: Send + Sync + Clone + 'static {
    async fn get_contribution(&self, id: i64) -> Result<Option<Contribution>, reject::Rejection>;
    async fn get_contributions(&self) -> Result<Vec<Contribution>, reject::Rejection>;
    async fn create_contribution(
        &self,
        contribution: ContributionRequest,
    ) -> Result<Contribution, reject::Rejection>;
    async fn delete_contribution(&self, id: i64) -> Result<(), reject::Rejection>;
}

#[async_trait]
impl DBContribution for DBAccess {
    async fn get_contribution(&self, id: i64) -> Result<Option<Contribution>, reject::Rejection> {
        let query = format!(
            "SELECT id FROM {} WHERE id = $1 ORDER BY created_at DESC",
            TABLE
        );
        match query_opt_timeout(self, query.as_str(), &[&id], DB_QUERY_TIMEOUT).await? {
            Some(contribution) => Ok(Some(row_to_contribution(&contribution))),
            None => Ok(None),
}
    }

    async fn get_contributions(&self) -> Result<Vec<Contribution>, reject::Rejection> {
        let query = format!("SELECT id FROM {} ORDER BY created_at DESC", TABLE);
        let rows = query_with_timeout(self, query.as_str(), &[], DB_QUERY_TIMEOUT).await?;
        Ok(rows.iter().map(row_to_contribution).collect())
    }

    async fn create_contribution(
        &self,
        contribution: ContributionRequest,
    ) -> Result<Contribution, reject::Rejection> {
        let query = format!("INSERT INTO {} (id) VALUES ($1) RETURNING *", TABLE);
        let row = query_one_timeout(self, &query, &[&contribution.id] ,DB_QUERY_TIMEOUT).await?;
        Ok(row_to_contribution(&row))
    }

    async fn delete_contribution(&self, id: i64) -> Result<(), reject::Rejection> {
        let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
        execute_query_with_timeout(self, &query, &[&id], DB_QUERY_TIMEOUT).await
    }
}

fn row_to_contribution(row: &Row) -> Contribution {
    let id: i64 = row.get(0);
    Contribution { id }
}
