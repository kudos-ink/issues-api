use mobc::async_trait;
use mobc_postgres::tokio_postgres::Row;
use warp::reject;

use crate::db::{
    errors::DBError,
    pool::{DBAccess, DBAccessor},
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
        let con = self.get_db_con().await?;
        let query = format!(
            "SELECT id FROM {} WHERE id = $1 ORDER BY created_at DESC",
            TABLE
        );
        let q = con.query_one(query.as_str(), &[&id]).await; //TODO: use this query_opt?
        match q {
            Ok(row) => Ok(Some(row_to_contribution(&row))),
            Err(_) => Ok(None),
        }
    }

    async fn get_contributions(&self) -> Result<Vec<Contribution>, reject::Rejection> {
        let con = self.get_db_con().await?;
        let query = format!("SELECT id FROM {} ORDER BY created_at DESC", TABLE);
        let q = con.query(query.as_str(), &[]).await;
        let rows = q.map_err(DBError::DBQuery)?;
        Ok(rows.iter().map(row_to_contribution).collect())
    }

    async fn create_contribution(
        &self,
        contribution: ContributionRequest,
    ) -> Result<Contribution, reject::Rejection> {
        let con = self.get_db_con().await?;
        let query = format!("INSERT INTO {} (id) VALUES ($1) RETURNING *", TABLE);
        let row = con
            .query_one(query.as_str(), &[&contribution.id])
            .await
            .map_err(|err| reject::custom(DBError::DBQuery(err)))?;

        Ok(row_to_contribution(&row))
    }

    async fn delete_contribution(&self, id: i64) -> Result<(), reject::Rejection> {
        let con = self.get_db_con().await?;
        let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
        con.query(query.as_str(), &[&id])
            .await
            .map_err(|err| reject::custom(DBError::DBQuery(err)))?;

        Ok(())
    }
}

fn row_to_contribution(row: &Row) -> Contribution {
    let id: i64 = row.get(0);
    Contribution { id }
}
