use diesel::prelude::*;
use crate::schema::{task_comments, users};
use crate::db::{errors::DBError, pool::{DBAccess, DBAccessor}};
use crate::api::users::models::User;
use super::models::{Comment, NewComment, CommentResponse};

pub trait DBComment: Send + Sync + Clone + 'static {
    fn by_task_id(&self, task_id_param: i32) -> Result<Vec<CommentResponse>, DBError>;
    fn create(&self, comment: &NewComment) -> Result<CommentResponse, DBError>;
}

impl DBComment for DBAccess {
    fn by_task_id(&self, task_id_param: i32) -> Result<Vec<CommentResponse>, DBError> {
        let conn = &mut self.get_db_conn();
        let comments_with_users = task_comments::table
            .inner_join(users::table.on(task_comments::user_id.eq(users::id)))
            .filter(task_comments::task_id.eq(task_id_param))
            .order(task_comments::created_at.asc())
            .select((Comment::as_select(), User::as_select()))
            .load::<(Comment, User)>(conn)?;

        let results = comments_with_users.into_iter().map(|(comment, user)| {
            CommentResponse {
                id: comment.id,
                content: comment.content,
                created_at: comment.created_at,
                user,
            }
        }).collect();

        Ok(results)
    }

    fn create(&self, comment: &NewComment) -> Result<CommentResponse, DBError> {
        let conn = &mut self.get_db_conn();
        let new_comment: Comment = diesel::insert_into(task_comments::table)
            .values(comment)
            .get_result(conn)
            .map_err(DBError::from)?;

        let user = users::table
            .find(new_comment.user_id)
            .first::<User>(conn)?;

         Ok(CommentResponse {
            id: new_comment.id,
            content: new_comment.content,
            created_at: new_comment.created_at,
            user,
        })
    }
}