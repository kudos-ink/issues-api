use mobc_postgres::tokio_postgres;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum DBError {
    #[error("error getting connection from DB pool: {0}")]
    DBPoolConnection(mobc::Error<tokio_postgres::Error>),
    #[error("error executing DB query: {0}")]
    DBQuery(#[from] tokio_postgres::Error),
    #[error("error creating table: {0}")]
    DBInit(tokio_postgres::Error),
    #[error("error reading file: {0}")]
    ReadFile(#[from] std::io::Error),
}

impl warp::reject::Reject for DBError {}