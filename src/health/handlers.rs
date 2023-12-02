use crate::db::{types::DBPool, self, utils::{execute_operation_with_timeout, DB_QUERY_TIMEOUT}};
use warp::{http::StatusCode, reject,  Reply, Rejection};

pub async fn health_handler(db_pool: DBPool) -> Result<impl Reply, Rejection> {
    let db_conn = db::pool::get_db_con(&db_pool).await.map_err(reject::custom)?;

    execute_operation_with_timeout(db_conn.execute("SELECT 1", &[]), DB_QUERY_TIMEOUT).await?;

    Ok(StatusCode::OK)
}