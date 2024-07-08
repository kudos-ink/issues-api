use dotenv::dotenv;
use serde_derive::{Deserialize, Serialize};
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
}

impl ApiConfig {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            http_server_host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned()),
            http_server_port: env::var("PORT")
                .unwrap_or_else(|_| "8000".to_owned())
                .parse()
                .expect("Invalid PORT"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PaginationParams {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default = "default_offset")]
    pub offset: i64,
}

fn default_limit() -> i64 {
    100 // Default limit
}

fn default_offset() -> i64 {
    0 // Default offset
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub total_count: Option<i64>,
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub data: Vec<T>,
}
