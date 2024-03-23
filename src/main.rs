use crate::types::ApiConfig;

mod auth;
mod db;
mod error;
mod handlers;
mod health;
mod issue;
mod organization;
mod repository;
mod tip;
mod types;
mod user;
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

    let db = utils::setup_db(&database_url).await;
    let app_filters = utils::setup_filters(db);

    let addr = format!("{}:{}", host, port)
        .parse::<std::net::SocketAddr>()
        .expect("Invalid server address");

    println!("listening on {}", addr);

    warp::serve(app_filters).run(addr).await;
}
