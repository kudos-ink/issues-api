use warp::Filter;
mod types;

use crate::types::ApiConfig;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let ApiConfig {
        http_server_host: host,
        http_server_port: port,
        database_url
    } = ApiConfig::new();

    // let db_pool = PgPoolOptions::new()
    //     .max_connections(1)
    //     .connect(&database_url)
    //     .await
    //     .expect("Failed to create the database pool.");
    let routes = warp::any().map(|| warp::reply::html("Hello, world!"));

    let addr = format!("{}:{}", host, port).parse::<std::net::SocketAddr>().expect("Invalid server address");

    warp::serve(routes).run(addr).await;
}
