use mobc_postgres::tokio_postgres::types::ToSql;
use tokio::time::{timeout, Duration};
use warp::reject;
use std::convert::Infallible;

use warp::Filter;

use super::{types::DBPool, errors::DBError, pool::get_db_con};

pub const DB_QUERY_TIMEOUT: Duration = Duration::from_secs(5);

pub fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub async fn execute_query_with_timeout(
    db_pool: &DBPool,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    timeout_duration: Duration,
) -> Result<(), reject::Rejection> {
    let db_conn = get_db_con(db_pool).await.map_err(reject::custom)?;

    timeout(timeout_duration, db_conn.execute(query, params))
        .await
        .map_err(|err| reject::custom(DBError::DBTimeout(err)))?
        .map_err(|err| reject::custom(DBError::DBQuery(err)))?;

    Ok(())
}