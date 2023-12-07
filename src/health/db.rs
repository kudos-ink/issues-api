use mobc::async_trait;
use warp::reject;

use crate::db::{
    pool::DBAccess,
    utils::{execute_with_timeout, DB_QUERY_TIMEOUT},
};

#[async_trait]
pub trait DBHealth: Send + Sync + Clone + 'static {
    async fn health(&self) -> Result<(), reject::Rejection>;
}

#[async_trait]
impl DBHealth for DBAccess {
    async fn health(&self) -> Result<(), reject::Rejection> {
        execute_with_timeout(self, "SELECT 1", &[], DB_QUERY_TIMEOUT).await?;
        Ok(())
    }
}
