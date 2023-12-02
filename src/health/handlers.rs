use crate::db::{types::DBPool, errors::DBError::DBQuery, self};
use warp::{http::StatusCode, reject,  Reply, Rejection};

pub async fn health_handler(db_pool: DBPool) -> Result<impl Reply, Rejection> {
    let db = db::pool::get_db_con(&db_pool).await.map_err(reject::custom)?;

    db.execute("SELECT 1", &[])
        .await
        .map_err(|err| reject::custom(DBQuery(err)))?;

    Ok(StatusCode::OK)
}