use chrono::{DateTime, Utc};
use mobc::async_trait;
use mobc_postgres::tokio_postgres::Row;
use warp::reject;

use crate::db::{pool::{DBAccess, DBAccessor}, errors::DBError};

use super::models::{ContributionRequest, Contribution};

const TABLE: &str = "contribution";

#[async_trait]
pub trait DBContribution: Send + Sync + Clone + 'static {
    // async fn get_contribution(&self, id: u64 ) -> Result<(), reject::Rejection>;
    // async fn get_contributions(&self) -> Result<(), reject::Rejection>;
    async fn create_contribution(&self, contribution: ContributionRequest) -> Result<Contribution, reject::Rejection>;
    // async fn delete_contribution(&self, id: u64) -> Result<(), reject::Rejection>;
}

#[async_trait]
impl DBContribution for DBAccess {
    // async fn health(&self) -> Result<(), reject::Rejection>{
    //     execute_query_with_timeout(self, "SELECT 1", &[], DB_QUERY_TIMEOUT).await?;
    //  Ok(())   
    // }

    async fn create_contribution(&self, contribution: ContributionRequest) -> Result<Contribution, reject::Rejection> {
        let con = self.get_db_con().await?;
        let query = format!("INSERT INTO {} (id) VALUES ($1) RETURNING *", TABLE);
        let map_err = con
                .query_one(query.as_str(), &[&contribution.id])
                .await
                .map_err(|err| reject::custom(DBError::DBQuery(err)));
        if map_err.is_err(){
            eprint!("{:?}", map_err)
        } else {
            eprint!("hola")
        }
        let row = map_err?;
        eprint!("{:?}", row);
        Ok(row_to_contribution(&row))
    }
}


fn row_to_contribution(row: &Row) -> Contribution {
    let id: i64 = row.get(0);
    Contribution {
        id
    }
}