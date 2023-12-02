use crate::db::{types::DBPool, utils::{execute_query_with_timeout, DB_QUERY_TIMEOUT}};
use warp::{http::StatusCode,  Reply, Rejection};

pub async fn health_handler(db_pool: DBPool) -> Result<impl Reply, Rejection> {
    execute_query_with_timeout(&db_pool, "SELECT 1", &[], DB_QUERY_TIMEOUT).await?;
    Ok(StatusCode::OK)
}