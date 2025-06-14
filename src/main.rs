mod types;
use log::{info, error};

use crate::types::{ApiConfig, NotificationsConfig};

mod api;
mod middlewares;
mod db;
mod errors;
pub mod schema;
mod utils;
mod email;

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

    let notifications_config = NotificationsConfig::new();
    if let Err(e) = notifications_config.validate() {
        error!("Invalid notifications configuration: {}", e);
        std::process::exit(1);
    }

    env_logger::init();

    let db = utils::setup_db(&database_url).await;
    let app_filters = utils::setup_filters(db.clone());

    if notifications_config.enabled {
        info!("Starting notification job");
        // Start the weekly notification job
        let sender_config = email::model::SMTPConfig {
            smtp_host: notifications_config.smtp_host,
            smtp_port: notifications_config.smtp_port,
            smtp_username: notifications_config.smtp_username,
            smtp_password: notifications_config.smtp_password,
            from_email: notifications_config.from_email,
        };

        // Spawn the notification job in a separate task
        tokio::spawn(email::notifications::start_notification_job(
            sender_config,
            db,
            notifications_config.days,
            notifications_config.subject,
            notifications_config.dry_run,
        ));
    }

    let addr = format!("{}:{}", host, port)
        .parse::<std::net::SocketAddr>()
        .expect("Invalid server address");

    info!("listening on {}", addr);

    warp::serve(app_filters).run(addr).await;
}
