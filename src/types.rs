use dotenv::dotenv;
use postgres_types::{FromSql, ToSql};
use serde_derive::{Deserialize, Serialize};
use std::{env, str::FromStr};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, FromSql, ToSql)]
pub struct IssueId(i32);

// TODO: remove the pub and use the IssueId en issues'endpoint
impl Into<i32> for IssueId {
    fn into(self) -> i32 {
        self.0
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, FromSql, ToSql)]
pub struct RepositoryId(i32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, FromSql, ToSql)]
pub struct TipId(i32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, FromSql, ToSql)]
pub struct UserId(i32);

#[derive(Debug, PartialEq, Eq)]
pub struct ParseIdError;

macro_rules! impl_from_str_for_id {
    ($t:ty) => {
        impl FromStr for $t {
            type Err = ParseIdError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.parse::<i32>()
                    .map(|id| Self(id))
                    .map_err(|_| ParseIdError)
            }
        }
    };
}

impl_from_str_for_id!(IssueId);
impl_from_str_for_id!(RepositoryId);
impl_from_str_for_id!(TipId);
impl_from_str_for_id!(UserId);

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
            http_server_host: env::var("HTTP_SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_owned()),
            http_server_port: env::var("HTTP_SERVER_PORT")
                .unwrap_or_else(|_| "8000".to_owned())
                .parse()
                .expect("Invalid HTTP_SERVER_PORT"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }
}
