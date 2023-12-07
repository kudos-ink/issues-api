use crate::{init_db, db::pool::DBAccess};


pub async fn get_db() -> DBAccess { //TODO: fix
    init_db(
        "postgres://postgres:password@localhost:5432/database".to_string(),
        "db.sql".to_string(),
    ).await
}