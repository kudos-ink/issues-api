use mobc::Pool;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, Error, NoTls};

use crate::db::types::{DBCon, DBPool};
use crate::db::errors::DBError;


const DB_POOL_MAX_OPEN: u64 = 32; // TODO: move to config
const DB_POOL_MAX_IDLE: u64 = 8;// TODO: move to config
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;// TODO: move to config


pub fn create_pool(database_url: &str) -> std::result::Result<DBPool, mobc::Error<Error>> {
    let config = Config::from_str(database_url)?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
            .max_open(DB_POOL_MAX_OPEN)
            .max_idle(DB_POOL_MAX_IDLE)
            .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
            .build(manager))
}

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon, DBError> {
    db_pool.get().await.map_err(DBError::DBPoolConnection)
}

pub async fn init_db(db_pool: &DBPool, sql_file: &str) -> Result<(), DBError> {
    let init_file = fs::read_to_string(sql_file)?;
    let con = get_db_con(db_pool).await?;
    con.batch_execute(init_file.as_str())
        .await
        .map_err(DBError::DBInit)?;
    Ok(())
}
