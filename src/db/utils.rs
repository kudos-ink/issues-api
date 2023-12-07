use std::time::Duration;

use mobc_postgres::tokio_postgres::{types::ToSql, Row};
use tokio::time::timeout;

use super::{errors::DBError, pool::DBAccessor};

pub const DB_QUERY_TIMEOUT: Duration = Duration::from_secs(5);
// TODO: improve next functions

pub async fn query_one_with_timeout(
    db_access: &impl DBAccessor,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    timeout_duration: Duration,
) -> Result<Row, DBError> {
    let db_conn = db_access.get_db_con().await?;

    timeout(timeout_duration, db_conn.query_one(query, params))
        .await
        .map_err(DBError::DBTimeout)?
        .map_err(DBError::DBQuery)
}

pub async fn query_with_timeout(
    db_access: &impl DBAccessor,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    timeout_duration: Duration,
) -> Result<Vec<Row>, DBError> {
    let db_conn = db_access.get_db_con().await?;

    timeout(timeout_duration, db_conn.query(query, params))
        .await
        .map_err(DBError::DBTimeout)?
        .map_err(DBError::DBQuery)
}

pub async fn execute_with_timeout(
    db_access: &impl DBAccessor,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    timeout_duration: Duration,
) -> Result<u64, DBError> {
    let db_conn = db_access.get_db_con().await?;

    timeout(timeout_duration, db_conn.execute(query, params))
        .await
        .map_err(DBError::DBTimeout)?
        .map_err(DBError::DBQuery)
}
