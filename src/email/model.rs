use diesel::{prelude::Queryable};
use lettre::AsyncSmtpTransport;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::db::pool::DBAccess;


#[derive(Debug, Deserialize)]
pub struct SMTPConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
}
pub struct EmailNotifier {
    pub config: SMTPConfig,
    pub mailer: AsyncSmtpTransport<lettre::Tokio1Executor>,
    pub db: DBAccess,
}


#[derive(
    Queryable, Debug, PartialEq, Deserialize, Serialize,
)]
pub struct NotificationData {
    pub github_id: Option<i64>,
    pub email: Option<String>,
    pub title: String,
    pub task_url: Option<String>,
    pub created_at: DateTime<Utc>,
}