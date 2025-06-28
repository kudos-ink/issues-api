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

#[derive(Deserialize, Debug, Clone)]
pub struct NotificationsConfig {
    pub days: i64,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub subject: String,
    pub enabled: bool,
    pub dry_run: bool,
}

impl NotificationsConfig {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            days: env::var("NOTIFICATIONS_DAYS").unwrap_or_else(|_| "30".to_owned()).parse().expect("NOTIFICATIONS_DAYS must be a number"),
            smtp_host: env::var("NOTIFICATIONS_SMTP_HOST").unwrap_or_else(|_| "".to_owned()),
            smtp_port: env::var("NOTIFICATIONS_SMTP_PORT").unwrap_or_else(|_| "0".to_owned()).parse().expect("NOTIFICATIONS_SMTP_PORT must be a number"),
            smtp_username: env::var("NOTIFICATIONS_SMTP_USERNAME").unwrap_or_else(|_| "".to_owned()),
            smtp_password: env::var("NOTIFICATIONS_SMTP_PASSWORD").unwrap_or_else(|_| "".to_owned()),
            from_email: env::var("NOTIFICATIONS_FROM_EMAIL").unwrap_or_else(|_| "".to_owned()),
            enabled: env::var("NOTIFICATIONS_ENABLED").unwrap_or_else(|_| "false".to_owned()).parse().expect("NOTIFICATIONS_ENABLED must be a boolean"),
            dry_run: env::var("NOTIFICATIONS_DRY_RUN").unwrap_or_else(|_| "true".to_owned()).parse().expect("NOTIFICATIONS_DRY_RUN must be a boolean"),
            subject: env::var("NOTIFICATIONS_SUBJECT").unwrap_or_else(|_| "Kudos Notifications Summary".to_owned()),
        }
    }
    pub fn validate(&self) -> Result<(), String> {
        if !self.enabled || self.dry_run {
            return Ok(());
        }
        if self.smtp_host.is_empty() {
            return Err("NOTIFICATIONS_SMTP_HOST must be set".to_owned());
        }
        if self.smtp_port == 0 {
            return Err("NOTIFICATIONS_SMTP_PORT must be set".to_owned());
        }
        if self.smtp_username.is_empty() {
            return Err("NOTIFICATIONS_SMTP_USERNAME must be set".to_owned());
        }
        if self.smtp_password.is_empty() {
            return Err("NOTIFICATIONS_SMTP_PASSWORD must be set".to_owned());
        }
        if self.from_email.is_empty() {
            return Err("NOTIFICATIONS_FROM_EMAIL must be set".to_owned());
        }
        Ok(())
    }
}