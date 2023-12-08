use super::{
    errors::DBError,
    pool::{self, DBAccess, DBAccessor},
};
use mobc_postgres::tokio_postgres::types::ToSql;
use tokio::time::{timeout, Duration};
use warp::reject;

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
pub async fn init_db(
    database_url: String,
    database_init_file: String,
) -> Result<DBAccess, DBError> {
    let db_pool = pool::create_pool(&database_url).map_err(DBError::DBPoolConnection)?;
    let db = DBAccess::new(db_pool);
    // TODO: use migrations
    db.init_db(&database_init_file).await?;
    Ok(db)
}
