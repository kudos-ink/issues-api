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

use super::models::{Issue, IssueCreate};
use super::utils::parse_github_issue_url;

const TABLE: &str = "issues";

#[async_trait]
pub trait DBIssue: Send + Sync + Clone + 'static {
    async fn get_issue(&self, id: i32) -> Result<Option<Issue>, reject::Rejection>;
    async fn get_issue_by_url(&self, url: &str) -> Result<Option<Issue>, reject::Rejection>;
    async fn get_issues(&self) -> Result<Vec<Issue>, reject::Rejection>;
    async fn create_issue(&self, issue: IssueCreate) -> Result<Issue, reject::Rejection>;
    async fn delete_issue(&self, id: i32) -> Result<(), reject::Rejection>;
}

#[async_trait]
impl DBIssue for DBAccess {
    async fn get_issue(&self, id: i32) -> Result<Option<Issue>, reject::Rejection> {
        let query = format!("SELECT * FROM {} WHERE id = $1", TABLE);
        match query_opt_timeout(self, query.as_str(), &[&id], DB_QUERY_TIMEOUT).await? {
            Some(user) => Ok(Some(row_to_issue(&user))),
            None => Ok(None),
        }
    }

    async fn get_issue_by_url(&self, url: &str) -> Result<Option<Issue>, reject::Rejection> {
        let query = format!("SELECT * FROM {} WHERE url = $1", TABLE);
        match query_opt_timeout(self, query.as_str(), &[&url], DB_QUERY_TIMEOUT).await? {
            Some(user) => Ok(Some(row_to_issue(&user))),
            None => Ok(None),
        }
    }

    async fn get_issues(&self) -> Result<Vec<Issue>, reject::Rejection> {
        let query = format!("SELECT * FROM {} ORDER BY created_at DESC", TABLE);
        let rows = query_with_timeout(self, query.as_str(), &[], DB_QUERY_TIMEOUT).await?;
        Ok(rows.iter().map(row_to_issue).collect())
    }

    async fn create_issue(&self, issue: IssueCreate) -> Result<Issue, reject::Rejection> {
        let query = format!(
            "INSERT INTO {} (issue_number, repository_id, url) VALUES ($1, $2, $3) RETURNING *",
            TABLE
        );
        let row = query_one_timeout(
            self,
            &query,
            &[&issue.id, &issue.repository_id, &issue.url],
            DB_QUERY_TIMEOUT,
        )
        .await?;
        Ok(row_to_issue(&row))
    }

    async fn delete_issue(&self, id: i32) -> Result<(), reject::Rejection> {
        let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
        execute_query_with_timeout(self, &query, &[&id], DB_QUERY_TIMEOUT).await
    }
}

fn row_to_issue(row: &Row) -> Issue {
    let id: i32 = row.get(0);
    let repository_id: i32 = row.get(1);
    // let url: &str = row.get(2); //TODO: define if we need it in the response
    Issue { id, repository_id }
}
