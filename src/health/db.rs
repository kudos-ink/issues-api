use diesel::sql_query;
use diesel::RunQueryDsl;

use crate::db::{
    errors::DBError,
    pool::{DBAccess, DBAccessor},
};

pub trait DBHealth: Send + Sync + Clone + 'static {
    fn health(&self) -> Result<(), DBError>;
}

impl DBHealth for DBAccess {
    fn health(&self) -> Result<(), DBError> {
        let conn = &mut self.get_db_conn();
        sql_query("SELECT 1")
            .execute(conn)
            .map_err(DBError::DBQuery)?;

        Ok(())
    }
}
