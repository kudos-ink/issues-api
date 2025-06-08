use lettre::AsyncSmtpTransport;
use serde::Deserialize;

use crate::db::pool::DBAccess;


#[derive(Debug, Deserialize)]
pub struct SenderConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
}
pub struct EmailNotifier {
    pub config: SenderConfig,
    pub mailer: AsyncSmtpTransport<lettre::Tokio1Executor>,
    pub db: DBAccess,
}
