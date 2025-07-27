use diesel::prelude::*;

use super::models::{Notification, DeleteNotification, NotificationResponse};
use crate::schema::{notifications::dsl as notifications_dsl, tasks::dsl as tasks_dsl, users::dsl as users_dsl, repositories::dsl as repositories_dsl, projects::dsl as projects_dsl };
use crate::api::tasks::models::{Task, TaskResponse};
use crate::api::users::models::User;
use crate::api::projects::models::{ProjectResponse, Project};
use crate::api::repositories::models::{RepositoryResponse, Repository};



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
            .inner_join(tasks_dsl::tasks
                .left_join(repositories_dsl::repositories.on(tasks_dsl::repository_id.eq(repositories_dsl::id.nullable())))
                .inner_join(projects_dsl::projects.on(repositories_dsl::project_id.eq(projects_dsl::id)))
                .left_join(users_dsl::users.on(tasks_dsl::assignee_user_id.eq(users_dsl::id.nullable())))
            )
            .filter(notifications_dsl::github_id.eq(github_id))
            .filter(notifications_dsl::seen.eq(seen))
            .select((
                notifications_dsl::notifications::all_columns(),
                tasks_dsl::tasks::all_columns(),
                repositories_dsl::repositories::all_columns().nullable(),
                projects_dsl::projects::all_columns(),
                users_dsl::users::all_columns().nullable(),
            ))
            .load::<(Notification, Task, Option<Repository>, Project, Option<User>)>(conn)
            .map_err(DBError::from)?
            .into_iter()
            .map(|(notification, task, repo, project, user)| NotificationResponse {
                id: notification.id,
                task_id: notification.task_id,
                task: TaskResponse {
                    id: task.id,
                    number: task.number,
                    repository_id: task.repository_id,
                    title: task.title,
                    description: task.description,
                    url: task.url,
                    labels: task.labels,
                    open: task.open,
                    type_: task.type_,
                    project_id: task.project_id,
                    created_by_user_id: task.created_by_user_id,
                    assignee_user_id: task.assignee_user_id,
                    user,
                    assignee_team_id: task.assignee_team_id,
                    funding_options: task.funding_options,
                    contact: task.contact,
                    skills: task.skills,
                    bounty: task.bounty,
                    approved_by: task.approved_by,
                    approved_at: task.approved_at,
                    status: task.status,
                    upvotes: task.upvotes,
                    user_vote: None, // TODO: return user votes if they exist
                    downvotes: task.downvotes,
                    is_featured: task.is_featured,
                    is_certified: task.is_certified,
                    featured_by_user_id: task.featured_by_user_id,
                    issue_created_at: task.issue_created_at,
                    issue_closed_at: task.issue_closed_at,
                    created_at: task.created_at,
                    updated_at: task.updated_at,
                    repository: repo.map(|r| RepositoryResponse {
                        id: r.id,
                        slug: r.slug,
                        name: r.name,
                        url: r.url,
                        language_slug: r.language_slug,
                        project: ProjectResponse {
                            id: project.id,
                            name: project.name,
                            slug: project.slug,
                            purposes: project.purposes,
                            stack_levels: project.stack_levels,
                            technologies: project.technologies,
                            avatar: project.avatar,
                            created_at: project.created_at,
                            updated_at: project.updated_at,
                            rewards: project.rewards,
                        },
                        created_at: r.created_at,
                        updated_at: r.updated_at,
                    }),
                },
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