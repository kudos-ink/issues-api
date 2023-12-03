mod types;

use db::pool::{DBAccess, DBAccessor};
use warp::Filter;

use crate::types::ApiConfig;

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
    let db_pool = db::pool::create_pool(&database_url).expect("database pool can be created");
    let db = DBAccess::new(db_pool);
    // TODO: use migrations
    db.init_db( &database_init_file)
        .await
        .expect("database can be initialized");

    let health_route = health::routes::routes(db);
    let error_handler = handlers::error_handler;

    // string all the routes together
    let routes = health_route
        .with(warp::cors().allow_any_origin())
        .recover(error_handler);

    let addr = format!("{}:{}", host, port)
        .parse::<std::net::SocketAddr>()
        .expect("Invalid server address");

    warp::serve(routes).run(addr).await;
}
