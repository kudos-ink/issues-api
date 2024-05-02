use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, PoolError};
use std::sync::Arc;
use std::time::Duration;

use crate::db::types::{DBConn, DBPool};

const DB_POOL_MAX_OPEN: u32 = 32; // TODO: move to config
const DB_POOL_MIN_IDLE: u32 = 8; // TODO: move to config
const DB_POOL_TIMEOUT_SECONDS: u64 = 15; // TODO: move to config

pub fn create_db_pool(database_url: &str) -> Result<DBPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .max_size(DB_POOL_MAX_OPEN)
        .min_idle(Some(DB_POOL_MIN_IDLE))
        .connection_timeout(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS))
        .build(manager)
}

pub trait DBAccessor: Send + Sync + Clone + 'static {
    fn new(db_pool: DBPool) -> Self;
    fn get_db_conn(&self) -> DBConn;
}

#[derive(Clone)]
pub struct DBAccess {
    pub db_pool: Arc<DBPool>,
}

impl DBAccessor for DBAccess {
    fn new(db_pool: DBPool) -> Self {
        Self {
            db_pool: Arc::new(db_pool),
        }
    }

    fn get_db_conn(&self) -> DBConn {
        self.db_pool.get().expect("Failed to get db connection")
    }
}
