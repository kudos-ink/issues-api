use mobc::async_trait;
use mobc_postgres::tokio_postgres::Row;
use warp::reject;

use crate::db::{
    pool::DBAccess,
    utils::{
        execute_query_with_timeout, query_one_timeout, query_opt_timeout, query_with_timeout,
        DB_QUERY_TIMEOUT,
    },
};

use super::models::{User, UserRequest};

const TABLE: &str = "users";

#[async_trait]
pub trait DBUser: Send + Sync + Clone + 'static {
    async fn get_user(&self, id: i32) -> Result<Option<User>, reject::Rejection>;
    async fn get_user_by_username(&self, username: &str)
        -> Result<Option<User>, reject::Rejection>;
    async fn get_users(&self) -> Result<Vec<User>, reject::Rejection>;
    async fn create_user(&self, user: UserRequest) -> Result<User, reject::Rejection>;
    async fn delete_user(&self, id: i32) -> Result<(), reject::Rejection>;
}

#[async_trait]
impl DBUser for DBAccess {
    async fn get_user(&self, id: i32) -> Result<Option<User>, reject::Rejection> {
        let query = format!("SELECT * FROM {} WHERE id = $1", TABLE);
        match query_opt_timeout(self, query.as_str(), &[&id], DB_QUERY_TIMEOUT).await? {
            Some(user) => Ok(Some(row_to_user(&user))),
            None => Ok(None),
        }
    }
    async fn get_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<User>, reject::Rejection> {
        let query = format!("SELECT * FROM {} WHERE username = $1", TABLE);
        match query_opt_timeout(self, query.as_str(), &[&username], DB_QUERY_TIMEOUT).await? {
            Some(user) => Ok(Some(row_to_user(&user))),
            None => Ok(None),
        }
    }

    async fn get_users(&self) -> Result<Vec<User>, reject::Rejection> {
        let query = format!("SELECT * FROM {} ORDER BY created_at DESC", TABLE);
        let rows = query_with_timeout(self, query.as_str(), &[], DB_QUERY_TIMEOUT).await?;
        Ok(rows.iter().map(row_to_user).collect())
    }

    async fn create_user(&self, user: UserRequest) -> Result<User, reject::Rejection> {
        let query = format!("INSERT INTO {} (username) VALUES ($1) RETURNING *", TABLE);
        let row = query_one_timeout(self, &query, &[&user.username], DB_QUERY_TIMEOUT).await?;
        Ok(row_to_user(&row))
    }

    async fn delete_user(&self, id: i32) -> Result<(), reject::Rejection> {
        let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
        execute_query_with_timeout(self, &query, &[&id], DB_QUERY_TIMEOUT).await
    }
}

fn row_to_user(row: &Row) -> User {
    let id: i32 = row.get(0);
    let username: &str = row.get(1);
    User {
        id,
        username: username.to_string(),
    }
}
