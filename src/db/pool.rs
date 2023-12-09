use mobc::{async_trait, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, Error, NoTls};

use crate::db::errors::DBError;
use crate::db::types::{DBCon, DBPool};

const DB_POOL_MAX_OPEN: u64 = 32; // TODO: move to config
const DB_POOL_MAX_IDLE: u64 = 8; // TODO: move to config
const DB_POOL_TIMEOUT_SECONDS: u64 = 15; // TODO: move to config

pub fn create_pool(database_url: &str) -> std::result::Result<DBPool, mobc::Error<Error>> {
    let config = Config::from_str(database_url)?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}

#[async_trait]
pub trait DBAccessor: Send + Sync + Clone + 'static {
    fn new(db_pool: DBPool) -> Self;
    async fn get_db_con(&self) -> Result<DBCon, DBError>;
    async fn init_db(&self, sql_file: &str) -> Result<(), DBError>;
}

#[derive(Clone)]
pub struct DBAccess {
    pub db_pool: DBPool,
}
#[async_trait]
impl DBAccessor for DBAccess {
    fn new(db_pool: DBPool) -> Self {
        Self { db_pool }
    }

    async fn get_db_con(&self) -> Result<DBCon, DBError> {
        self.db_pool.get().await.map_err(DBError::DBPoolConnection)
    }

    async fn init_db(&self, sql_file: &str) -> Result<(), DBError> {
        let init_file = fs::read_to_string(sql_file)?;
        let con = self.get_db_con().await?;
        con.batch_execute(init_file.as_str())
            .await
            .map_err(DBError::DBInit)?;
        Ok(())
    }
}
