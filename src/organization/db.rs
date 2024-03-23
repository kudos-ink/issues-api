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

use super::models::{Organization, OrganizationCreateRequest};

const TABLE: &str = "organizations";

#[async_trait]
pub trait DBOrganization: Send + Sync + Clone + 'static {
    async fn get_organization(&self, id: i32) -> Result<Option<Organization>, reject::Rejection>;
    async fn get_organization_by_name(
        &self,
        name: &str,
    ) -> Result<Option<Organization>, reject::Rejection>;
    async fn get_organizations(&self, query: Option<String>) -> Result<Vec<Organization>, reject::Rejection>;
    async fn create_organization(
        &self,
        organization: OrganizationCreateRequest,
    ) -> Result<Organization, reject::Rejection>;
    async fn delete_organization(&self, id: i32) -> Result<(), reject::Rejection>;
}

#[async_trait]
impl DBOrganization for DBAccess {
    async fn get_organization(&self, id: i32) -> Result<Option<Organization>, reject::Rejection> {
        let query = format!("SELECT * FROM {} WHERE id = $1", TABLE);
        match query_opt_timeout(self, query.as_str(), &[&id], DB_QUERY_TIMEOUT).await? {
            Some(organization) => Ok(Some(row_to_organization(&organization))),
            None => Ok(None),
        }
    }
    async fn get_organization_by_name(
        &self,
        name: &str,
    ) -> Result<Option<Organization>, reject::Rejection> {
        let query = format!("SELECT * FROM {} WHERE name = $1", TABLE);
        match query_opt_timeout(self, query.as_str(), &[&name], DB_QUERY_TIMEOUT).await? {
            Some(organization) => Ok(Some(row_to_organization(&organization))),
            None => Ok(None),
        }
    }

    async fn get_organizations(&self, name: Option<String>) -> Result<Vec<Organization>, reject::Rejection> {
        let where_clause = match name {
            Some(_) => "WHERE name like $1",
            None => "",
        };

        let query = format!("SELECT * FROM {} {} ORDER BY created_at DESC", TABLE, where_clause);
        let rows = match name {
            Some(value) => query_with_timeout(self, query.as_str(), &[&value], DB_QUERY_TIMEOUT).await?,
            None => query_with_timeout(self, query.as_str(), &[], DB_QUERY_TIMEOUT).await?,
        };

        Ok(rows.iter().map(row_to_organization).collect())
    }

    async fn create_organization(
        &self,
        organization: OrganizationCreateRequest,
    ) -> Result<Organization, reject::Rejection> {
        let query = format!("INSERT INTO {} (name) VALUES ($1) RETURNING *", TABLE);
        let row = query_one_timeout(self, &query, &[&organization.name], DB_QUERY_TIMEOUT).await?;
        Ok(row_to_organization(&row))
    }

    async fn delete_organization(&self, id: i32) -> Result<(), reject::Rejection> {
        let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
        execute_query_with_timeout(self, &query, &[&id], DB_QUERY_TIMEOUT).await
    }
}

fn row_to_organization(row: &Row) -> Organization {
    let id: i32 = row.get(0);
    let name: &str = row.get(1);
    Organization {
        id,
        name: name.to_string(),
    }
}
