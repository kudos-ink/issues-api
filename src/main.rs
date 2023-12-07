mod types;

use db::pool::{DBAccess, DBAccessor};
use warp::Filter;

use crate::types::ApiConfig;

mod contributions;
mod db;
mod handlers;
mod health;

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
        database_init_file,
    } = ApiConfig::new();

    // init db
    let db = init_db(database_url, database_init_file).await;

    let health_route = health::routes::routes(db.clone());
    let contribution_route = contributions::routes::routes(db);
    let error_handler = handlers::error_handler;

    // string all the routes together
    let routes = health_route
        .or(contribution_route)
        .with(warp::cors().allow_any_origin())
        .recover(error_handler);

    let addr = format!("{}:{}", host, port)
        .parse::<std::net::SocketAddr>()
        .expect("Invalid server address");

    warp::serve(routes).run(addr).await;
}

async fn init_db(database_url: String, database_init_file: String) -> DBAccess {
    let db_pool = db::pool::create_pool(&database_url).expect("database pool can be created");
    let db = DBAccess::new(db_pool);
    // TODO: use migrations
    db.init_db(&database_init_file)
        .await
        .expect("database cannot be initialized");
    db
}
