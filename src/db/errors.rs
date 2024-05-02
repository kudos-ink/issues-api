use diesel::r2d2::PoolError;
use diesel::result::Error as DieselError;
use thiserror::Error;
use tokio::time::error::Elapsed;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("error getting connection from DB pool: {0}")]
    DBPoolConnection(PoolError),
    #[error("error executing DB query: {0}")]
    DBQuery(#[from] DieselError),
    #[error("error initializing the database: {0}")]
    DBInit(DieselError),
    #[error("error reading file: {0}")]
    ReadFile(#[from] std::io::Error),
    #[error("database operation timed out: {0}")]
    DBTimeout(#[from] Elapsed),
}

impl warp::reject::Reject for DBError {}
