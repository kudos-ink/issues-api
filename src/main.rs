mod types;
use log::{info, warn};

use crate::types::ApiConfig;

mod api;
mod middlewares;
mod db;
mod errors;
pub mod schema;
mod utils;

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

    env_logger::init();

    let db = utils::setup_db(&database_url).await;
    let app_filters = utils::setup_filters(db);

    let addr = format!("{}:{}", host, port)
        .parse::<std::net::SocketAddr>()
        .expect("Invalid server address");

    info!("listening on {}", addr);
    warn!("listening on {}", addr);

    warp::serve(app_filters).run(addr).await;
}
