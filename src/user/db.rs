use crate::{
    db::{
        pool::DBAccess,
        utils::{
            execute_query_with_timeout, query_one_timeout, query_opt_timeout, query_with_timeout,
            DB_QUERY_TIMEOUT,
        },
    },
    pagination::GetPagination,
};
use mobc::async_trait;
use mobc_postgres::tokio_postgres::Row;
use warp::reject;

use super::models::{NewUser, PatchUser, User, UserSort, UsersRelations};

const TABLE: &str = "users";

#[async_trait]
pub trait DBUser: Send + Sync + Clone + 'static {
    async fn get_user(
        &self,
        id: i32,
        relations: UsersRelations,
    ) -> Result<Option<User>, reject::Rejection>;
    async fn get_user_by_username(
        &self,
        username: &str,
        relations: UsersRelations,
    ) -> Result<Option<User>, reject::Rejection>;
    async fn get_users(
        &self,
        relations: UsersRelations,
        pagination: GetPagination,
        sort: UserSort,
    ) -> Result<Vec<User>, reject::Rejection>;
    async fn create_user(&self, user: NewUser) -> Result<User, reject::Rejection>;
    async fn update_user_maintainers(
        &self,
        id: i32,
        user: PatchUser,
    ) -> Result<User, reject::Rejection>;
    async fn delete_user(&self, id: i32) -> Result<(), reject::Rejection>;
}

#[async_trait]
impl DBUser for DBAccess {
    async fn get_user(
        &self,
        id: i32,
        relations: UsersRelations,
    ) -> Result<Option<User>, reject::Rejection> {
        let mut query = format!("SELECT * FROM {} ", TABLE);
        if relations.maintainers {
            query += "LEFT JOIN maintainers on maintainers.user_id = users.id ";
            query += "LEFT JOIN repositories on maintainers.repository_id = repositories.id ";
        }
        if relations.issues {
            query += "LEFT JOIN issues on issues.user_id = users.id ";
            if relations.tips {
                query += "LEFT JOIN tips on tips.id = issues.id ";
            }
        }
        if relations.wishes {
            query += "LEFT JOIN comments on comments.user_id = users.id ";
            query += "LEFT JOIN wishes on wishes.id = comments.wish_id ";
        }
        query += "WHERE id = $1";

        match query_opt_timeout(self, query.as_str(), &[&id], DB_QUERY_TIMEOUT).await? {
            Some(user) => Ok(Some(row_to_user(&user))),
            None => Ok(None),
        }
    }
    async fn get_user_by_username(
        &self,
        username: &str,
        relations: UsersRelations,
    ) -> Result<Option<User>, reject::Rejection> {
        let mut query = format!("SELECT * FROM {} ", TABLE);
        if relations.maintainers {
            query += "LEFT JOIN maintainers on maintainers.user_id = users.id ";
            query += "LEFT JOIN repositories on maintainers.repository_id = repositories.id ";
        }
        if relations.issues {
            query += "LEFT JOIN issues on issues.user_id = users.id ";
            if relations.tips {
                query += "LEFT JOIN tips on tips.id = issues.id ";
            }
        }
        if relations.wishes {
            query += "LEFT JOIN comments on comments.user_id = users.id ";
            query += "LEFT JOIN wishes on wishes.id = comments.wish_id ";
        }
        query += "WHERE username = $1";
        match query_opt_timeout(self, query.as_str(), &[&username], DB_QUERY_TIMEOUT).await? {
            Some(user) => Ok(Some(row_to_user(&user))),
            None => Ok(None),
        }
    }

    async fn get_users(
        &self,
        relations: UsersRelations,
        pagination: GetPagination,
        sort: UserSort,
    ) -> Result<Vec<User>, reject::Rejection> {
        let mut query = format!("SELECT * FROM {} ", TABLE);
        if relations.maintainers {
            query += "LEFT JOIN maintainers on maintainers.user_id = users.id ";
            query += "LEFT JOIN repositories on maintainers.repository_id = repositories.id ";
        }
        if relations.issues {
            query += "LEFT JOIN issues on issues.user_id = users.id ";
            if relations.tips {
                query += "LEFT JOIN tips on tips.id = issues.id ";
            }
        }
        if relations.wishes {
            query += "LEFT JOIN comments on comments.user_id = users.id ";
            query += "LEFT JOIN wishes on wishes.id = comments.wish_id ";
        }

        query += &format!("ORDER BY {} {}", sort.field, sort.order); // cannot use $1 or $2

        query += "LIMIT $1 OFFSET $2";
        let rows = query_with_timeout(
            self,
            query.as_str(),
            &[&pagination.limit, &pagination.offset],
            DB_QUERY_TIMEOUT,
        )
        .await?;
        Ok(rows.iter().map(row_to_user).collect())
    }

    async fn create_user(&self, user: NewUser) -> Result<User, reject::Rejection> {
        let query = format!("INSERT INTO {} (username) VALUES ($1) RETURNING *", TABLE);
        let row = query_one_timeout(self, &query, &[&user.username], DB_QUERY_TIMEOUT).await?;
        let new_user = row_to_user(&row);

        if let Some(repositories) = user.repositories {
            for repo_id in repositories {
                let query =
                    "INSERT INTO maintainers (user_id, repository_id) VALUES ($1, $2)".to_string();
                query_one_timeout(self, &query, &[&new_user.id, &repo_id], DB_QUERY_TIMEOUT)
                    .await?;
            }
        }
        Ok(new_user)
    }
    async fn update_user_maintainers(
        &self,
        id: i32,
        user: PatchUser,
    ) -> Result<User, reject::Rejection> {
        let query = "DELETE maintainers WHERE user_ID = $1".to_string();
        let row = query_one_timeout(self, &query, &[&id], DB_QUERY_TIMEOUT).await?;
        for repo_id in user.repositories {
            let query =
                "INSERT INTO maintainers (user_id, repository_id) VALUES ($1, $2)".to_string();
            query_one_timeout(self, &query, &[&id, &repo_id], DB_QUERY_TIMEOUT).await?;
        }
        // TODO: make a db tx
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
