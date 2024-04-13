use dotenv::dotenv;
use std::env;

/// Configuration used by this API.
#[derive(Debug, Clone)]
pub struct ApiConfig {
    /// HTTP server host name (default: 127.0.0.1).
    pub http_server_host: String,
    /// HTTP server port (default: 8000).
    pub http_server_port: u16,
    /// Database URL.
    pub database_url: String,
    /// Database init file.
    pub database_init_file: String,
}

impl ApiConfig {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            http_server_host: env::var("HTTP_SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_owned()),
            http_server_port: env::var("HTTP_SERVER_PORT")
                .unwrap_or_else(|_| "8000".to_owned())
                .parse()
                .expect("Invalid HTTP_SERVER_PORT"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            database_init_file: env::var("DATABASE_INIT_FILE").unwrap_or_else(|_| "".to_owned()),
        }
    }
}
