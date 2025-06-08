use diesel::prelude::*;

use super::models::{Notification, DeleteNotification, NotificationResponse};
use crate::schema::{notifications::dsl as notifications_dsl, tasks::dsl as tasks_dsl};
use crate::api::tasks::models::{Task};

use crate::db::{
    errors::DBError,
    pool::{DBAccess, DBAccessor},
};

pub trait DBNotification: Send + Sync + Clone + 'static {
    fn by_github_id(&self, github_id: i64, seen: bool) -> Result<Vec<NotificationResponse>, DBError>;
    fn delete(&self, notification: &DeleteNotification) -> Result<(), DBError>;
    fn delete_all(&self, github_id: i64) -> Result<(), DBError>;
}

impl DBNotification for DBAccess {
    fn by_github_id(&self, github_id: i64, seen: bool) -> Result<Vec<NotificationResponse>, DBError> {
        let conn = &mut self.get_db_conn();

        let result = notifications_dsl::notifications
            .inner_join(tasks_dsl::tasks)
            .filter(notifications_dsl::github_id.eq(github_id))
            .filter(notifications_dsl::seen.eq(seen))
            .select((
                notifications_dsl::notifications::all_columns(),
                tasks_dsl::tasks::all_columns(),
            ))
            .load::<(Notification, Task)>(conn)
            .map_err(DBError::from)?
            .into_iter()
            .map(|(notification, task)| NotificationResponse {
                id: notification.id,
                task_id: notification.task_id,
                task: task.into(),
                created_at: notification.created_at,
            })
            .collect();

        Ok(result)
    }

    fn delete(&self, notification: &DeleteNotification) -> Result<(), DBError> {
        let conn = &mut self.get_db_conn();
        let github_id = notification.github_id.ok_or_else(|| {
            DBError::DBQuery(diesel::result::Error::NotFound)
        })?;
        
        diesel::update(
            notifications_dsl::notifications
                .filter(notifications_dsl::github_id.eq(github_id))
                .filter(notifications_dsl::id.eq(notification.id))
        )
        .set(notifications_dsl::seen.eq(true))
        .execute(conn)
        .map_err(DBError::from)?;

        Ok(())
    }

    fn delete_all(&self, github_id: i64) -> Result<(), DBError> {
        let conn = &mut self.get_db_conn();
        diesel::delete(notifications_dsl::notifications.filter(notifications_dsl::github_id.eq(github_id)))
            .execute(conn)
            .map_err(DBError::from)?;
        Ok(())
    }
}