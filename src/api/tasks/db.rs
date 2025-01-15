use diesel::prelude::*;

use crate::schema::tasks::dsl as tasks_dsl;
use crate::schema::tasks_votes::dsl as tasks_votes_dsl;

use crate::db::{
    errors::DBError,
    pool::{DBAccess, DBAccessor},
};
use crate::types::PaginationParams;
use crate::utils;

use super::models::{NewTask, QueryParams, Task, TaskVote, TaskVoteDB, UpdateTask};
pub trait DBTask: Send + Sync + Clone + 'static {
    fn all(
        &self,
        params: QueryParams,
        pagination: PaginationParams,
    ) -> Result<(Vec<Task>, i64), DBError>;
    fn by_id(&self, id: i32) -> Result<Option<Task>, DBError>;
    fn create(&self, role: &NewTask) -> Result<Task, DBError>;
    fn update(&self, id: i32, role: &UpdateTask) -> Result<Task, DBError>;
    fn delete(&self, id: i32) -> Result<(), DBError>;
    fn add_vote_to_task(&self, task_user: &TaskVoteDB) -> Result<TaskVote, DBError>;
    fn delete_task_vote(&self, id: i32) -> Result<(), DBError>;
}

impl DBTask for DBAccess {
    fn all(
        &self,
        params: QueryParams,
        pagination: PaginationParams,
    ) -> Result<(Vec<Task>, i64), DBError> {
        let conn = &mut self.get_db_conn();

        let build_query = || {
            let mut query = tasks_dsl::tasks.into_boxed();

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

        let result = build_query()
            .order(tasks_dsl::created_at.desc())
            .offset(pagination.offset)
            .limit(pagination.limit)
            .load::<Task>(conn)?;

        Ok((result, total_count))
    }

    fn by_id(&self, id: i32) -> Result<Option<Task>, DBError> {
        let conn = &mut self.get_db_conn();
        let result = tasks_dsl::tasks
            .find(id)
            .first::<Task>(conn)
            .optional()
            .map_err(DBError::from)?;
        Ok(result)
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
        let vote= diesel::insert_into(tasks_votes_dsl::tasks_votes)
            .values(task_vote)
            .get_result(conn)
            .map_err(DBError::from)?;

        Ok(vote)
    }
    fn delete_task_vote(&self, id: i32) -> Result<(), DBError> {
        let conn = &mut self.get_db_conn();
        diesel::delete(tasks_votes_dsl::tasks_votes.filter(tasks_votes_dsl::id.eq(id)))
            .execute(conn)
            .map_err(DBError::from)?;

        Ok(())
    }


}
