use ::warp::Reply;
use warp::{filters::BoxedFilter, Filter};

use crate::{
    api::{health, issues, projects, repositories, users},
    db::{
        self,
        errors::DBError,
        pool::{DBAccess, DBAccessor},
    },
    errors::error_handler,
};

pub async fn setup_db(url: &str) -> DBAccess {
    let db_pool = db::pool::create_db_pool(url)
        .map_err(DBError::DBPoolConnection)
        .expect("Failed to create DB pool");

    // TODO: Extend this helper in tests by incorporating DB migration with Diesel

    // // In Cargo.toml
    // [dependencies]
    // diesel_migrations = "1.4.0"

    // // In main.rs
    //#[macro_use]
    // extern crate diesel_migrations;

    // embed_migrations!();

    // // Get a database connection from the pool
    // let conn = db_pool.get()
    //     .expect("Failed to get database connection from pool");

    // // Run embedded migrations
    // embedded_migrations::run(&conn)
    //     .expect("Failed to run database migrations");

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
    s.split(",").map(|el: &str| el.to_string()).collect()
}
