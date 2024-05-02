use ::warp::Reply;
use warp::{filters::BoxedFilter, Filter};

use crate::{
    db::{
        self,
        errors::DBError,
        pool::{DBAccess, DBAccessor},
    },
    error_handler::error_handler,
    health,
};

pub async fn setup_db(url: &String) -> DBAccess {
    let db_pool = db::pool::create_db_pool(&url)
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
    // let repositories_route = repository::routes::routes(db.clone());

    let error_handler = error_handler;

    health_route
        // .or(repositories_route)
        .with(warp::cors().allow_any_origin())
        .recover(error_handler)
        .boxed()
}

pub fn parse_ids(s: &str) -> Vec<i32> {
    s.split(",").map(|id| id.parse::<i32>().unwrap()).collect()
}
