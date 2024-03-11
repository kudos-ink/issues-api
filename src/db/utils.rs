use std::{
    env::{self, VarError},
    io::Error,
};

use super::{
    errors::DBError,
    pool::{self, DBAccess, DBAccessor},
};
use mobc_postgres::tokio_postgres::{types::ToSql, Row};
use tokio::time::{timeout, Duration};
use warp::reject;

pub const DB_QUERY_TIMEOUT: Duration = Duration::from_secs(5);

pub async fn query_with_timeout(
    db_access: &impl DBAccessor,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    timeout_duration: Duration,
) -> Result<Vec<Row>, reject::Rejection> {
    let db_conn = db_access.get_db_con().await.map_err(reject::custom)?;

    timeout(timeout_duration, db_conn.query(query, params))
        .await
        .map_err(|err| reject::custom(DBError::DBTimeout(err)))?
        .map_err(|err| reject::custom(DBError::DBQuery(err)))
}

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

pub async fn query_opt_timeout(
    db_access: &impl DBAccessor,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    timeout_duration: Duration,
) -> Result<Option<Row>, reject::Rejection> {
    let db_conn = db_access.get_db_con().await.map_err(reject::custom)?;

    timeout(timeout_duration, db_conn.query_opt(query, params))
        .await
        .map_err(|err| reject::custom(DBError::DBTimeout(err)))?
        .map_err(|err| reject::custom(DBError::DBQuery(err)))
}

pub async fn query_one_timeout(
    db_access: &impl DBAccessor,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    timeout_duration: Duration,
) -> Result<Row, reject::Rejection> {
    let db_conn = db_access.get_db_con().await.map_err(reject::custom)?;

    timeout(timeout_duration, db_conn.query_one(query, params))
        .await
        .map_err(|err| reject::custom(DBError::DBTimeout(err)))?
        .map_err(|err| reject::custom(DBError::DBQuery(err)))
}

pub async fn init_db(database_url: String) -> Result<DBAccess, DBError> {
    let db_pool = pool::create_pool(&database_url).map_err(DBError::DBPoolConnection)?;
    let db = DBAccess::new(db_pool);
    Ok(db)
}
