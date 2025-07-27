use diesel::prelude::*;
use crate::schema::{task_comments, users};
use crate::db::{errors::DBError, pool::{DBAccess, DBAccessor}};
use crate::api::users::models::User;
use super::models::{Comment, NewComment, CommentResponse};
use crate::schema::task_comments::dsl as comments_dsl;
use chrono::Utc;

pub trait DBComment: Send + Sync + Clone + 'static {
    fn by_task_id(&self, task_id_param: i32) -> Result<Vec<CommentResponse>, DBError>;
    fn by_comment_id(&self, id: i32) -> Result<Option<CommentResponse>, DBError>;
    fn create(&self, comment: &NewComment) -> Result<CommentResponse, DBError>;
    fn has_replies(&self, comment_id: i32) -> Result<bool, DBError>;
    fn hard_delete(&self, id: i32) -> Result<usize, DBError>;
    fn soft_delete(&self, id: i32) -> Result<Comment, DBError>;
}

impl DBComment for DBAccess {
    fn by_task_id(&self, task_id_param: i32) -> Result<Vec<CommentResponse>, DBError> {
        let conn = &mut self.get_db_conn();
        let comments_with_users = comments_dsl::task_comments
            .inner_join(users::table.on(comments_dsl::user_id.eq(users::id)))
            .filter(comments_dsl::task_id.eq(task_id_param))
            .order(comments_dsl::created_at.asc())
            .select((Comment::as_select(), User::as_select()))
            .load::<(Comment, User)>(conn)?;

        let results = comments_with_users.into_iter().map(|(comment, user)| {
            if comment.status == "deleted" {
                CommentResponse {
                    id: comment.id,
                    status: comment.status,
                    content: "[deleted]".to_string(),
                    created_at: comment.created_at,
                    parent_comment_id: comment.parent_comment_id,
                    user: User {
                        id: -1,
                        username: "[deleted]".to_string(),
                        avatar: None,
                        github_id: None,
                        created_at: Utc::now(),
                        updated_at: None,
                        email: None,
                        email_notifications_enabled: false,
                    },
                }
            } else {
                CommentResponse {
                    id: comment.id,
                    status: comment.status,
                    content: comment.content,
                    created_at: comment.created_at,
                    parent_comment_id: comment.parent_comment_id,
                    user,
                }
            }
        }).collect();
        Ok(results)
    }

    fn by_comment_id(&self, id: i32) -> Result<Option<CommentResponse>, DBError> {
        let conn = &mut self.get_db_conn();
        let result = task_comments::table
            .inner_join(users::table.on(task_comments::user_id.eq(users::id)))
            .filter(task_comments::id.eq(id))
            .select((Comment::as_select(), User::as_select()))
            .first::<(Comment, User)>(conn)
            .optional()?;

        Ok(result.map(|(comment, user)| 
            if comment.status == "deleted" {
                CommentResponse {
                    id: comment.id,
                    status: comment.status,
                    content: "[deleted]".to_string(),
                    created_at: comment.created_at,
                    parent_comment_id: comment.parent_comment_id,
                    user: User {
                        id: -1,
                        username: "[deleted]".to_string(),
                        avatar: None,
                        github_id: None,
                        created_at: Utc::now(),
                        updated_at: None,
                        email: None,
                        email_notifications_enabled: false,
                    },
                }
            } else {
                CommentResponse {
                    id: comment.id,
                    status: comment.status,
                    content: comment.content,
                    created_at: comment.created_at,
                    parent_comment_id: comment.parent_comment_id,
                    user,
                }
            }
    
        ))
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
            parent_comment_id: new_comment.parent_comment_id,
            status: new_comment.status
        })
    }

    fn has_replies(&self, comment_id: i32) -> Result<bool, DBError> {
        let conn = &mut self.get_db_conn();
        comments_dsl::task_comments
            .filter(comments_dsl::parent_comment_id.eq(comment_id))
            .select(diesel::dsl::count_star())
            .first::<i64>(conn)
            .map(|count| count > 0)
            .map_err(DBError::from)
    }

    fn hard_delete(&self, id: i32) -> Result<usize, DBError> {
        let conn = &mut self.get_db_conn();
        diesel::delete(comments_dsl::task_comments.find(id)).execute(conn).map_err(DBError::from)
    }

    fn soft_delete(&self, id: i32) -> Result<Comment, DBError> {
        let conn = &mut self.get_db_conn();

        diesel::update(comments_dsl::task_comments.find(id))
            .set((
                comments_dsl::status.eq("deleted"),
            ))
            .get_result(conn)
            .map_err(DBError::from)
    }
}