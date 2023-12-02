use crate::db::{types::DBPool, errors::DBError, self};
use warp::{http::StatusCode, reject,  Reply, Rejection};
use tokio::time::{timeout, Duration};

const QUERY_TIMEOUT: Duration = Duration::from_secs(5);

pub async fn health_handler(db_pool: DBPool) -> Result<impl Reply, Rejection> {
    let db_conn = db::pool::get_db_con(&db_pool).await.map_err(reject::custom)?;

    timeout(QUERY_TIMEOUT, db_conn.execute("SELECT 1", &[]))
        .await
        .map_err(|err| reject::custom(DBError::DBTimeout(err)))? // Handle the timeout error
        .map_err(|err| reject::custom(DBError::DBQuery(err)))?; // Handle the query error

    Ok(StatusCode::OK)
}