use mobc_postgres::tokio_postgres::types::ToSql;
use tokio::time::{timeout, Duration};
use warp::reject;
use super::{errors::DBError, pool::DBAccessor};

pub const DB_QUERY_TIMEOUT: Duration = Duration::from_secs(5);

pub async fn execute_query_with_timeout(
    db_access: &impl DBAccessor,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    timeout_duration: Duration,
) -> Result<(), reject::Rejection> {
    let db_conn = db_access.get_db_con().await.map_err(reject::custom)?;

    timeout(timeout_duration, db_conn.execute(query, params))
        .await
        .map_err(|err| reject::custom(DBError::DBTimeout(err)))?
        .map_err(|err| reject::custom(DBError::DBQuery(err)))?;

    Ok(())
}