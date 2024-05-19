use std::env;

use crate::{
    api::{health, issues, projects, repositories, users},
    db::{
        self,
        errors::DBError,
        pool::{DBAccess, DBAccessor},
    },
    errors::error_handler,
};
use ::warp::Reply;
use diesel::RunQueryDsl;
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use warp::{filters::BoxedFilter, Filter};

pub async fn setup_db(url: &str) -> DBAccess {
    let db_pool = db::pool::create_db_pool(url)
        .map_err(DBError::DBPoolConnection)
        .expect("Failed to create DB pool");
    DBAccess::new(db_pool)
}

pub fn setup_filters(db: DBAccess) -> BoxedFilter<(impl Reply,)> {
    let health_route = health::routes::routes(db.clone());
    let projects_route = projects::routes::routes(db.clone());
    let repositories_route = repositories::routes::routes(db.clone());
    let issues_route = issues::routes::routes(db.clone());
    let users_route = users::routes::routes(db.clone());

    health_route
        .or(projects_route)
        .or(repositories_route)
        .or(issues_route)
        .or(users_route)
        .with(warp::cors().allow_any_origin())
        .recover(error_handler)
        .boxed()
}

pub fn parse_ids(s: &str) -> Vec<i32> {
    let mut ids = Vec::new();
    for token in s.split_whitespace() {
        if let Ok(id) = token.parse::<i32>() {
            ids.push(id);
        }
    }
    ids
}

pub fn parse_comma_values(s: &str) -> Vec<String> {
    s.split(',').map(|el: &str| el.to_string()).collect()
}

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
