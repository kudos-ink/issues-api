use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use lettre::{
    message::{ Message, MultiPart, SinglePart},
    transport::smtp::AsyncSmtpTransport,
    AsyncTransport, Tokio1Executor,
};
use log::{error, info};

use crate::{db::pool::{DBAccess, DBAccessor}, email::model::NotificationData, schema::{notification_schedule, notifications, projects, repositories, tasks, users}};
use crate::email::model::{EmailNotifier, SMTPConfig};

impl EmailNotifier {
    pub fn new(config: SMTPConfig, db: DBAccess) -> Self {
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp_host)
            .expect("Failed to create SMTP transport")
            .port(config.smtp_port)
            .credentials(lettre::transport::smtp::authentication::Credentials::new(
                config.smtp_username.clone(),
                config.smtp_password.clone(),
            ))
            .build();

        Self { config, mailer, db }
    }

    pub async fn send_notifications(&self, dry_run: bool, days: i64, subject: String) -> Result<(), Box<dyn std::error::Error>> {
        let conn = &mut self.db.get_db_conn();
        let one_week_ago = Utc::now() - Duration::days(days);
        info!("Getting notifications from the last {} days", days);
        // Get all users with their unread notifications from the last week
        let notifications = notifications::table
            .inner_join(
                users::table.on(notifications::github_id.nullable().eq(users::github_id.nullable()))
            )
            .inner_join(tasks::table.on(notifications::task_id.eq(tasks::id)))
            .left_join(repositories::table.on(tasks::repository_id.eq(repositories::id.nullable())))
            .left_join(projects::table.on(tasks::project_id.eq(projects::id.nullable())))
            .filter(notifications::created_at.gt(one_week_ago))
            .filter(notifications::seen.eq(false))
            .select((
                users::github_id,
                users::email,
                tasks::title,
                tasks::url,
                notifications::created_at,
            ))
            .load::<NotificationData>(conn)?;

        // Group notifications by user
        let mut user_notifications: std::collections::HashMap<i64, Vec<(String, Option<String>, DateTime<Utc>)>> =
            std::collections::HashMap::new();
        let mut user_emails: std::collections::HashMap<i64, String> = std::collections::HashMap::new();

        for notification in notifications {
            if let (Some(github_id), Some(email)) = (notification.github_id, notification.email) {
                info!("Adding notification for user {} with email {}", github_id, email);
                user_notifications
                    .entry(github_id)
                    .or_default()
                    .push((notification.title, notification.task_url, notification.created_at));
                user_emails.insert(github_id, email);
            }
        }

        // Send email to each user
        for (github_id, notifications) in user_notifications {
            if let Some(user_email) = user_emails.get(&github_id) {
                info!("Sending email to {}", user_email);
                
                // Create HTML content
                let mut html_content = String::from(
                    r#"<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
        .notification-list { list-style-type: none; padding: 0; }
        .notification-item { margin-bottom: 15px; padding: 10px; border-left: 3px solid #007bff; }
        .notification-title { font-weight: bold; color: #007bff; }
        .notification-time { color: #666; font-size: 0.9em; }
        .notification-url { color: #28a745; text-decoration: none; }
        .notification-url:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <h2>Your Unread Notifications</h2>
    <div class="notification-list">
"#);

                for (title, url, created_at) in notifications.clone() {
                    html_content.push_str(&format!(
                        r#"        <div class="notification-item">
            <div class="notification-title">{}</div>
            <div class="notification-time">Created at {}</div>"#,
                        title,
                        created_at.format("%Y-%m-%d %H:%M:%S UTC")
                    ));
                    if let Some(url) = url {
                        html_content.push_str(&format!(
                            r#"            <div><a href="{}" class="notification-url">{}</a></div>"#,
                            url, url
                        ));
                    }
                    html_content.push_str("</div>\n");
                }

                html_content.push_str(
                    r#"    </div>
</body>
</html>"#);

                // Create plain text version as fallback
                let mut text_content = String::from("Here are your unread notifications from the past week:\n\n");
                for (title, url, created_at) in notifications {
                    text_content.push_str(&format!(
                        "- {} (created at {})\n",
                        title,
                        created_at.format("%Y-%m-%d %H:%M:%S UTC")
                    ));
                    if let Some(url) = url {
                        text_content.push_str(&format!("    - {}\n", url));
                    }
                }

                let email = Message::builder()
                    .from(self.config.from_email.parse()?)
                    .to(user_email.parse()?)
                    .subject(subject.clone())
                    .multipart(
                        MultiPart::alternative()
                            .singlepart(SinglePart::plain(text_content.clone()))
                            .singlepart(SinglePart::html(html_content))
                    )?;

                if dry_run {
                    info!("Dry run: Would have sent email to {}: {}", user_email, text_content);
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

pub async fn start_notification_job(config: SMTPConfig, db: DBAccess, days: i64, subject: String, dry_run: bool) {
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
        info!("Next run: {}", next_run);
        let mut sleep_duration = next_run.signed_duration_since(now);
        // If next_run is in the past, immediately schedule the next run
        if now >= next_run && last_run.is_none() {
            info!("Sending notifications of {:?}", next_run);
            if let Err(e) = notifier.send_notifications(dry_run, days, subject.clone()).await {
                error!("Failed to send notifications: {}", e);
            }

            // Calculate and save next run time
            let next_run = now + Duration::days(days);
            info!("Updating notification schedule to run at {}", next_run);
            sleep_duration = next_run.signed_duration_since(now);

            // Update the existing record instead of inserting a new one
            diesel::update(notification_schedule::table)
                .filter(notification_schedule::id.eq(schedule.0))
                .set((
                    notification_schedule::updated_at.eq(&now),
                    notification_schedule::last_run.eq(&now),
                ))
                .execute(conn)
                .expect("Failed to update notification schedule");
            
            // Insert a new record for the next run
            diesel::insert_into(notification_schedule::table)
                .values((
                    notification_schedule::next_run.eq(&next_run),
                    notification_schedule::created_at.eq(&now),
                ))
                .execute(conn)
                .expect("Failed to create notification schedule");
        }

        info!("Sleeping for {} seconds", sleep_duration.num_seconds());
        tokio::time::sleep(sleep_duration.to_std().unwrap()).await;
    }
} 