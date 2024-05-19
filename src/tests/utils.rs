use std::env;

use crate::{
    db::pool::{DBAccess, DBAccessor},
    utils::setup_db,
};
use diesel::RunQueryDsl;
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_random_database_name() -> String {
    let rng = thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(10)
        .collect();
    format!("test_db_{}", random_string)
}

pub async fn generate_test_database() -> DBAccess {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
    let database_url = env::var("DATABASE_URL").expect("missing DATABASE");
    let database_name = generate_random_database_name();
    let db = setup_db(&database_url).await;
    let conn = &mut db.get_db_conn();
    diesel::sql_query(format!("CREATE DATABASE {}", database_name))
        .execute(conn)
        .expect("Failed to create database");
    db.get_db_conn()
        .run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");
    db
}
