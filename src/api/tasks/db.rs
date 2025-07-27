use diesel::prelude::*;

use crate::schema::tasks::dsl as tasks_dsl;
use crate::schema::users::dsl as users_dsl;
use crate::schema::repositories::dsl as repositories_dsl;
use crate::schema::projects::dsl as projects_dsl;
use crate::schema::tasks_votes::dsl as tasks_votes_dsl;
use diesel::pg::upsert::excluded;

use crate::db::{
    errors::DBError,
    pool::{DBAccess, DBAccessor},
};
use crate::types::PaginationParams;
use crate::utils;
use crate::api::users::models::User;
use crate::api::projects::models::{ProjectResponse, Project};
use crate::api::repositories::models::{RepositoryResponse, Repository};


use super::models::{NewTask, QueryParams, Task, TaskVote, TaskVoteDB, UpdateTask, TaskResponse};
pub trait DBTask: Send + Sync + Clone + 'static {
    fn all(
        &self,
        params: QueryParams,
        pagination: PaginationParams,
        current_user_id: Option<i32>
    ) -> Result<(Vec<TaskResponse>, i64), DBError>;
    fn by_id(&self, id: i32, current_user_id: Option<i32>) -> Result<Option<TaskResponse>, DBError>;
    fn create(&self, role: &NewTask) -> Result<Task, DBError>;
    fn update(&self, id: i32, role: &UpdateTask) -> Result<Task, DBError>;
    fn delete(&self, id: i32) -> Result<(), DBError>;
    fn add_vote_to_task(&self, task_user: &TaskVoteDB) -> Result<TaskVote, DBError>;
    // fn delete_task_vote(&self, id: i32) -> Result<(), DBError>;
    fn delete_vote_by_user_and_task(&self, user_id: i32, task_id: i32) -> Result<usize, DBError>;
}

impl DBTask for DBAccess {
    fn all(
        &self,
        params: QueryParams,
        pagination: PaginationParams,
        current_user_id: Option<i32>,
    ) -> Result<(Vec<TaskResponse>, i64), DBError> {
        let conn = &mut self.get_db_conn();

        let build_query = || {
            let mut query = tasks_dsl::tasks
                .left_join(
                    repositories_dsl::repositories
                        .on(tasks_dsl::repository_id.eq(repositories_dsl::id.nullable()))
                )
                .left_join(
                    projects_dsl::projects
                        .on(tasks_dsl::project_id.eq(projects_dsl::id.nullable()))
                )
                .left_join(
                    users_dsl::users
                        .on(tasks_dsl::assignee_user_id.eq(users_dsl::id.nullable()))
                )
                .left_join(tasks_votes_dsl::tasks_votes.on(
                    tasks_votes_dsl::task_id.eq(tasks_dsl::id)
                    .and(tasks_votes_dsl::user_id.eq(current_user_id.unwrap_or(-1)))
                ))
                .into_boxed();
            

            if let Some(repository_id) = params.repository_id {
                query = query.filter(tasks_dsl::repository_id.eq(repository_id));
            }
            if let Some(labels) = params.labels.as_ref() {
                query = query.filter(tasks_dsl::labels.overlaps_with(utils::parse_comma_values(labels)));
            }
            if let Some(open) = params.open {
                query = query.filter(tasks_dsl::open.eq(open));
            }
            if let Some(type_) = params.type_.as_ref() {
                query = query.filter(tasks_dsl::type_.eq(type_));
            }
            if let Some(project_id) = params.project_id {
                query = query.filter(tasks_dsl::project_id.eq(project_id));
            }
            if let Some(created_by_user_id) = params.created_by_user_id {
                query = query.filter(tasks_dsl::created_by_user_id.eq(created_by_user_id));
            }
            if let Some(assignee_user_id) = params.assignee_user_id {
                query = query.filter(tasks_dsl::assignee_user_id.eq(assignee_user_id));
            }
            if let Some(assignee_team_id) = params.assignee_team_id {
                query = query.filter(tasks_dsl::assignee_team_id.eq(assignee_team_id));
            }
            if let Some(funding_options) = params.funding_options.as_ref() {
                query = query.filter(tasks_dsl::funding_options.overlaps_with(utils::parse_comma_values(funding_options)));
            }
            if let Some(contact) = params.contact.as_ref() {
                query = query.filter(tasks_dsl::contact.eq(contact));
            }
            if let Some(skills) = params.skills.as_ref() {
                query = query.filter(tasks_dsl::skills.overlaps_with(utils::parse_comma_values(skills)));
            }
            if let Some(bounty) = params.bounty {
                query = query.filter(tasks_dsl::bounty.eq(bounty));
            }
            if let Some(approved_at) = params.approved_at {
                query = query.filter(tasks_dsl::approved_at.eq(approved_at));
            }
            if let Some(status) = params.status.as_ref() {
                query = query.filter(tasks_dsl::status.eq(status));
            }
            if let Some(upvotes) = params.upvotes {
                query = query.filter(tasks_dsl::upvotes.ge(upvotes));
            }
            if let Some(downvotes) = params.downvotes {
                query = query.filter(tasks_dsl::downvotes.le(downvotes));
            }
            if let Some(is_featured) = params.is_featured {
                query = query.filter(tasks_dsl::is_featured.eq(is_featured));
            }
            if let Some(is_certified) = params.is_certified {
                query = query.filter(tasks_dsl::is_certified.eq(is_certified));
            }
            if let Some(featured_by_user_id) = params.featured_by_user_id {
                query = query.filter(tasks_dsl::featured_by_user_id.eq(featured_by_user_id));
            }
            if let Some(issue_created_at) = params.issue_created_at {
                query = query.filter(tasks_dsl::issue_created_at.ge(issue_created_at));
            }
            if let Some(issue_closed_at) = params.issue_closed_at {
                query = query.filter(tasks_dsl::issue_closed_at.le(issue_closed_at));
            }

            query
        };


        let total_count = build_query().count().get_result::<i64>(conn)?;

        let query = build_query()
            .select((
                (tasks_dsl::tasks::all_columns()),
                (repositories_dsl::repositories::all_columns().nullable()),
                (projects_dsl::projects::all_columns().nullable()),
                (users_dsl::users::all_columns().nullable()),
                (tasks_votes_dsl::vote.nullable())
            ))
            .order(tasks_dsl::created_at.desc())
            .offset(pagination.offset)
            .limit(pagination.limit);

        let rows = query.load::<(Task, Option<Repository>, Option<Project>, Option<User>, Option<i32>)>(conn)?;

        let tasks_with_assignee = rows
        .into_iter()
        .map(|(task, repo, project, user, user_vote)| TaskResponse { 
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
            repository: match (repo, project) {
                (Some(r), Some(p)) => Some(RepositoryResponse {
                    id: r.id,
                    slug: r.slug,
                    name: r.name,
                    url: r.url,
                    language_slug: r.language_slug,
                    project: ProjectResponse {
                        id: p.id,
                        name: p.name,
                        slug: p.slug,
                        purposes: p.purposes,
                        stack_levels: p.stack_levels,
                        technologies: p.technologies,
                        avatar: p.avatar,
                        created_at: p.created_at,
                        updated_at: p.updated_at,
                        rewards: p.rewards,
                    },
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                }),
                _ => None,
            },
            assignee_team_id: task.assignee_team_id,
            funding_options: task.funding_options,
            contact: task.contact,
            skills: task.skills,
            bounty: task.bounty,
            approved_by: task.approved_by,
            approved_at: task.approved_at,
            status: task.status,
            upvotes: task.upvotes,
            downvotes: task.downvotes,
            user_vote,
            is_featured: task.is_featured,
            is_certified: task.is_certified,
            featured_by_user_id: task.featured_by_user_id,
            issue_created_at: task.issue_created_at,
            issue_closed_at: task.issue_closed_at,
            created_at: task.created_at,
            updated_at: task.updated_at,
        })
        .collect();

        Ok((tasks_with_assignee, total_count))
    }

    fn by_id(&self, id: i32,  current_user_id: Option<i32>) -> Result<Option<TaskResponse>, DBError> {
        let conn = &mut self.get_db_conn();
    
        let row = tasks_dsl::tasks

            .left_join(
                repositories_dsl::repositories
                    .on(tasks_dsl::repository_id.eq(repositories_dsl::id.nullable()))
            )

            .left_join(
                    projects_dsl::projects
                        .on(tasks_dsl::project_id.eq(projects_dsl::id.nullable()))
                )

            .left_join(
                users_dsl::users
                    .on(tasks_dsl::assignee_user_id.eq(users_dsl::id.nullable()))
            )
            .left_join(tasks_votes_dsl::tasks_votes.on(
                tasks_votes_dsl::task_id.eq(tasks_dsl::id)
                .and(tasks_votes_dsl::user_id.eq(current_user_id.unwrap_or(-1)))
            ))
            .filter(tasks_dsl::id.eq(id))
            .select((
                (tasks_dsl::tasks::all_columns()),
                (repositories_dsl::repositories::all_columns().nullable()),
                (projects_dsl::projects::all_columns().nullable()),
                (users_dsl::users::all_columns().nullable()),
                (tasks_votes_dsl::vote.nullable())
            ))
            .first::<(Task, Option<Repository>, Option<Project>, Option<User>, Option<i32>)>(conn)
            .optional()?;
    
        Ok(row.map(|(task, repo, project, user, user_vote)| TaskResponse {
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
            downvotes: task.downvotes,
            user_vote,
            is_featured: task.is_featured,
            is_certified: task.is_certified,
            featured_by_user_id: task.featured_by_user_id,
            issue_created_at: task.issue_created_at,
            issue_closed_at: task.issue_closed_at,
            created_at: task.created_at,
            updated_at: task.updated_at,
            repository: match (repo, project) {
                (Some(r), Some(p)) => Some(RepositoryResponse {
                    id: r.id,
                    slug: r.slug,
                    name: r.name,
                    url: r.url,
                    language_slug: r.language_slug,
                    project: ProjectResponse {
                        id: p.id,
                        name: p.name,
                        slug: p.slug,
                        purposes: p.purposes,
                        stack_levels: p.stack_levels,
                        technologies: p.technologies,
                        avatar: p.avatar,
                        created_at: p.created_at,
                        updated_at: p.updated_at,
                        rewards: p.rewards,
                    },
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                }),
                _ => None,
            },
        }))
    }
    
    

    fn create(&self, task: &NewTask) -> Result<Task, DBError> {
        let conn = &mut self.get_db_conn();
        let task = diesel::insert_into(tasks_dsl::tasks)
            .values(task)
            .get_result(conn)
            .map_err(DBError::from)?;

        Ok(task)
    }

    fn update(&self, id: i32, task: &UpdateTask) -> Result<Task, DBError> {
        let conn = &mut self.get_db_conn();
        let task = diesel::update(tasks_dsl::tasks.filter(tasks_dsl::id.eq(id)))
            .set((task, tasks_dsl::updated_at.eq(diesel::dsl::now)))
            .get_result::<Task>(conn)
            .map_err(DBError::from)?;

        Ok(task)
    }

    fn delete(&self, id: i32) -> Result<(), DBError> {
        let conn = &mut self.get_db_conn();
        diesel::delete(tasks_dsl::tasks.filter(tasks_dsl::id.eq(id)))
            .execute(conn)
            .map_err(DBError::from)?;

        Ok(())
    }

    fn add_vote_to_task(&self, task_vote: &TaskVoteDB) -> Result<TaskVote, DBError> {
        let conn = &mut self.get_db_conn();

        let vote = diesel::insert_into(tasks_votes_dsl::tasks_votes)
            .values(task_vote)
            .on_conflict((tasks_votes_dsl::user_id, tasks_votes_dsl::task_id))
            .do_update()
            .set(tasks_votes_dsl::vote.eq(excluded(tasks_votes_dsl::vote)))
            .get_result(conn)
            .map_err(DBError::from)?;

        Ok(vote)
    }
    
    // fn delete_task_vote(&self, id: i32) -> Result<(), DBError> {
    //     let conn = &mut self.get_db_conn();
    //     diesel::delete(tasks_votes_dsl::tasks_votes.filter(tasks_votes_dsl::id.eq(id)))
    //         .execute(conn)
    //         .map_err(DBError::from)?;

    //     Ok(())
    // }

    fn delete_vote_by_user_and_task(&self, user_id_param: i32, task_id_param: i32) -> Result<usize, DBError> {
        let conn = &mut self.get_db_conn();
        diesel::delete(
            tasks_votes_dsl::tasks_votes
                .filter(tasks_votes_dsl::user_id.eq(user_id_param))
                .filter(tasks_votes_dsl::task_id.eq(task_id_param))
        )
        .execute(conn)
        .map_err(DBError::from)
    }


}
