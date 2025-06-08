use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use lettre::{
    message::{header, Message},
    transport::smtp::AsyncSmtpTransport,
    AsyncTransport, Tokio1Executor,
};
use log::{error, info};

use crate::{db::pool::{DBAccess, DBAccessor}, schema::{notifications, tasks, users, notification_schedule}};
use crate::email::model::{EmailNotifier, SenderConfig};

impl EmailNotifier {
    pub fn new(config: SenderConfig, db: DBAccess) -> Self {
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host)
            .expect("Failed to create SMTP transport")
            .port(config.smtp_port)
            .credentials(lettre::transport::smtp::authentication::Credentials::new(
                config.smtp_username.clone(),
                config.smtp_password.clone(),
            ))
            .build();

        Self { config, mailer, db }
    }

    pub async fn send_notifications(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
        let conn = &mut self.db.get_db_conn();
        let one_week_ago = Utc::now() - Duration::weeks(1);

        // Get all users with their unread notifications from the last week
        let notifications = notifications::table
            .inner_join(
                users::table.on(notifications::github_id.nullable().eq(users::github_id.nullable()))
            )
            .inner_join(tasks::table.on(notifications::task_id.eq(tasks::id)))
            .filter(notifications::created_at.gt(one_week_ago))
            .filter(notifications::seen.eq(false))
            .select((
                users::github_id,
                users::email,
                tasks::title,
                notifications::created_at,
            ))
            .load::<(Option<i64>, Option<String>, String, DateTime<Utc>)>(conn)?;

        // Group notifications by user
        let mut user_notifications: std::collections::HashMap<i64, Vec<(String, DateTime<Utc>)>> =
            std::collections::HashMap::new();
        let mut user_emails: std::collections::HashMap<i64, String> = std::collections::HashMap::new();

        for (github_id, email, title, created_at) in notifications {
            if let (Some(github_id), Some(email)) = (github_id, email) {
                user_notifications
                    .entry(github_id)
                    .or_default()
                    .push((title, created_at));
                user_emails.insert(github_id, email);
            }
        }

        // Send email to each user
        for (github_id, notifications) in user_notifications {
            if let Some(user_email) = user_emails.get(&github_id) {
                let mut email_content = String::from(
                    "Here are your unread notifications from the past week:\n\n",
                );

                for (title, created_at) in notifications {
                    email_content.push_str(&format!(
                        "- {} (created at {})\n",
                        title,
                        created_at.format("%Y-%m-%d %H:%M:%S UTC")
                    ));
                }
                let email = Message::builder()
                    .from(self.config.from_email.parse()?)
                    .to(user_email.parse()?)
                    .subject("Your Weekly Notifications Summary")
                    .header(header::ContentType::TEXT_PLAIN)
                    .body(email_content.clone())?;

                if dry_run {
                    info!("Dry run: Would have sent email to {}: {}", user_email, email_content);
                    continue;
                }
                match self.mailer.send(email).await {
                    Ok(_) => info!("Successfully sent weekly notification email to {}", user_email),
                    Err(e) => error!("Failed to send email to {}: {}", user_email, e),
                }
            }
        }

        Ok(())
    }
}

pub async fn start_notification_job(config: SenderConfig, db: DBAccess, days: i64, dry_run: bool) {
    let notifier = EmailNotifier::new(config, db.clone());
    
    loop {
        let conn = &mut db.get_db_conn();
        let now = Utc::now();

        // Get the next run time from the database
        let schedule = notification_schedule::table
            .order_by(notification_schedule::id.desc())
            .first::<(i32, DateTime<Utc>, Option<DateTime<Utc>>, DateTime<Utc>, DateTime<Utc>)>(conn)
            .expect("Failed to get notification schedule");

        let next_run = schedule.1;
        let last_run = schedule.2;

        if now >= next_run && (last_run.is_none() || last_run.unwrap() < next_run) {
            info!("Sending notifications");
            if let Err(e) = notifier.send_notifications(dry_run).await {
                error!("Failed to send notifications: {}", e);
            }

            // Calculate and save next run time
            let next_run = now + Duration::days(days);
            let next_run = next_run
                .date_naive()
                .and_hms_opt(9, 0, 0)
                .unwrap()
                .and_utc();

            // Update the existing record instead of inserting a new one
            diesel::update(notification_schedule::table)
                .filter(notification_schedule::id.eq(schedule.0))
                .set((
                    notification_schedule::next_run.eq(&next_run),
                    notification_schedule::last_run.eq(&now),
                    notification_schedule::updated_at.eq(&now),
                ))
                .execute(conn)
                .expect("Failed to update notification schedule");
        }

        // Sleep until next run or 1 hour, whichever is sooner
        let sleep_duration = std::cmp::min(
            next_run.signed_duration_since(now),
            Duration::hours(1),
        );
        tokio::time::sleep(sleep_duration.to_std().unwrap()).await;
    }
} 