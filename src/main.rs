mod types;

use std::{
    env::{self, VarError},
    process::exit,
};

use db::utils::init_db;
use warp::Filter;

use crate::{db::pool::DBAccessor, types::ApiConfig};

mod auth;
mod db;
mod error;
mod handlers;
mod health;
mod issue;
mod organization;
mod repository;
mod tip;
mod user;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let ApiConfig {
        http_server_host: host,
        http_server_port: port,
        database_url,
    } = ApiConfig::new();

    // init db
    let db = init_db(database_url).await.unwrap(); //If there's an error the api should panic
                                                   // migration and exit
    if let Ok(db_file) = env::var("DATABASE_INIT_FILE") {
        // If present, run the migration and exit
        match db.init_db(&db_file).await {
            Ok(_) => exit(0),
            Err(_) => exit(1),
        }
    }
    let health_route = health::routes::routes(db.clone());
    let users_route = user::routes::routes(db.clone());
    let organizations_route = organization::routes::routes(db.clone());
    let repositories_route = repository::routes::routes(db.clone());
    let tips_route = tip::routes::routes(db);
    //TODO: add issue route
    let error_handler = handlers::error_handler;

    // string all the routes together
    let routes = health_route
        .or(users_route)
        .or(organizations_route)
        .or(repositories_route)
        .or(tips_route)
        .with(warp::cors().allow_any_origin())
        .recover(error_handler);

    let addr = format!("{}:{}", host, port)
        .parse::<std::net::SocketAddr>()
        .expect("Invalid server address");

    println!("listening on {}", addr);

    warp::serve(routes).run(addr).await;
}
