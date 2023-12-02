use tokio::time::{timeout, Duration};
use warp::reject;
use std::convert::Infallible;

use warp::Filter;

use super::{types::DBPool, errors::DBError};

pub const DB_QUERY_TIMEOUT: Duration = Duration::from_secs(5);

pub fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub async fn execute_operation_with_timeout<F, T, E>(future: F, duration: Duration) -> Result<T, reject::Rejection>
where
    F: std::future::Future<Output = Result<T, E>>,
    E: Into<DBError>,
{
    match timeout(duration, future).await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(err)) => Err(reject::custom(err.into())),
        Err(_elapsed) => Err(reject::custom(DBError::DBTimeout(_elapsed))),
    }
}