use chrono::NaiveDateTime;
use mobc::async_trait;
use mobc_postgres::tokio_postgres::Row;
use postgres_types::ToSql;
use warp::reject;

use crate::{
    db::{
        pool::DBAccess,
        utils::{
            execute_query_with_timeout, query_one_timeout, query_opt_timeout, query_with_timeout,
            DB_QUERY_TIMEOUT,
        },
    },
    types::{IssueId, RepositoryId, TipId, UserId},
};

use super::{
    errors::TipError,
    models::{CreateTipRequest, Tip, TipStatus, TipType, UpdateTipRequest},
};

const TABLE: &str = "tips";

#[async_trait]
pub trait DBTip: Send + Sync + Clone + 'static {
    /// Fetches a tip by its ID.
    async fn get_tip(&self, id: TipId) -> Result<Option<Tip>, reject::Rejection>;
    /// Fetches the tip of a given issue ID.
    async fn get_tip_by_issue(&self, issue_id: IssueId) -> Result<Option<Tip>, reject::Rejection>;
    /// Fetches all tips for a given repository ID.
    async fn get_tips_by_repository(
        &self,
        repository_id: RepositoryId,
    ) -> Result<Vec<Tip>, reject::Rejection>;
    /// Creates a single tip.
    async fn create_tip(&self, input: CreateTipRequest) -> Result<Tip, reject::Rejection>;
    /// Updates a single tip by its ID.
    async fn update_tip(
        &self,
        id: TipId,
        params: UpdateTipRequest,
    ) -> Result<Option<Tip>, reject::Rejection>;
    /// Deletes a single tip by its ID.
    async fn delete_tip(&self, id: TipId) -> Result<(), reject::Rejection>;
}

#[async_trait]
impl DBTip for DBAccess {
    async fn get_tip(&self, id: TipId) -> Result<Option<Tip>, reject::Rejection> {
        let query = format!("SELECT * FROM {} WHERE id = $1", TABLE);
        match query_opt_timeout(self, query.as_str(), &[&id], DB_QUERY_TIMEOUT).await? {
            Some(tip) => Ok(Some(row_to_tip(&tip))),
            None => Ok(None),
        }
    }

    async fn get_tip_by_issue(&self, issue_id: IssueId) -> Result<Option<Tip>, reject::Rejection> {
        let query = format!("SELECT * FROM {} WHERE issue_id = $1", TABLE);
        match query_opt_timeout(self, query.as_str(), &[&issue_id], DB_QUERY_TIMEOUT).await? {
            Some(tip) => Ok(Some(row_to_tip(&tip))),
            None => Ok(None),
        }
    }

    async fn get_tips_by_repository(
        &self,
        repository_id: RepositoryId,
    ) -> Result<Vec<Tip>, reject::Rejection> {
        let query = format!(
            "SELECT * FROM {} WHERE repository_id = $1 ORDER BY created_at DESC",
            TABLE
        );
        let rows =
            query_with_timeout(self, query.as_str(), &[&repository_id], DB_QUERY_TIMEOUT).await?;
        Ok(rows.iter().map(row_to_tip).collect())
    }

    async fn create_tip(&self, input: CreateTipRequest) -> Result<Tip, reject::Rejection> {
        let query = format!("INSERT INTO {} (tip_type, amount, issue_id, contributor_id, curator_id) VALUES ($1, $2, $3, $4, $5) RETURNING *", TABLE);
        let amount_str = input.amount.to_string();
        
        let row = query_one_timeout(
            self,
            &query,
            &[
                &input.tip_type as &(dyn ToSql + Sync),
                &amount_str,
                &input.issue_id as &(dyn ToSql + Sync),
                &input.contributor_id as &(dyn ToSql + Sync),
                &input.curator_id as &(dyn ToSql + Sync),
            ],
            DB_QUERY_TIMEOUT,
        )
        .await?;

        Ok(row_to_tip(&row))
    }

    async fn update_tip(
        &self,
        id: TipId,
        params: UpdateTipRequest,
    ) -> Result<Option<Tip>, reject::Rejection> {
        let existing_tip = match self.get_tip(id).await? {
            Some(tip) => tip,
            None => return Err(warp::reject::custom(TipError::NotFound)),
        };

        let payload = Tip {
            id: existing_tip.id,
            status: params.status.unwrap_or(existing_tip.status),
            tip_type: params.tip_type.unwrap_or(existing_tip.tip_type),
            amount: params.amount.unwrap_or(existing_tip.amount),
            to: params.to.unwrap_or(existing_tip.to),
            from: params.from.unwrap_or(existing_tip.from),
            issue_id: existing_tip.issue_id,
            contributor_id: existing_tip.contributor_id,
            curator_id: existing_tip.curator_id,
            created_at: existing_tip.created_at,
            updated_at: chrono::Utc::now().naive_utc(),
        };

        let query = format!(
            "UPDATE tips SET status = $1, tip_type = $2, amount = $3, \"to\" = $4, \"from\" = $5, updated_at = $6 WHERE id = $7 RETURNING *;",
        );
        let amount_str = payload.amount.to_string();
        let to_slice = &payload.to[..];
        let from_slice = &payload.from[..];

        let row = query_one_timeout(
            self, 
            &query, 
            &[
                &payload.status as &(dyn ToSql + Sync),
                &payload.tip_type as &(dyn ToSql + Sync),
                &amount_str,
                &to_slice,
                &from_slice,
                &payload.updated_at as &(dyn ToSql + Sync),
                &id as &(dyn ToSql + Sync),
            ],
            DB_QUERY_TIMEOUT,
        ).await?;

        Ok(Some(row_to_tip(&row)))
    }

    async fn delete_tip(&self, id: TipId) -> Result<(), reject::Rejection> {
        let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
        execute_query_with_timeout(self, &query, &[&id], DB_QUERY_TIMEOUT).await
    }
}

fn row_to_tip(row: &Row) -> Tip {
    let id: TipId = row.get(0);
    let status: TipStatus = row.get(1);
    let tip_type: TipType = row.get(2);
    let amount_str: &str = row.get(3);
    let to_slice: &[u8] = row.get(4);
    let from_slice: &[u8] = row.get(5);
    let issue_id: IssueId = row.get(6);
    let contributor_id: UserId = row.get(7);
    let curator_id: UserId = row.get(8);
    let created_at: NaiveDateTime = row.get(9);
    let updated_at: NaiveDateTime = row.get(10);

    let amount: u128 = amount_str
        .parse::<u128>()
        .expect("Failed to convert 'amount' into a u128 value");
    let to: [u8; 32] = to_slice
        .try_into()
        .expect("Failed to convert 'to' into a fixed-size array");
    let from: [u8; 32] = from_slice
        .try_into()
        .expect("Failed to convert 'from' into a fixed-size array");

    Tip {
        id,
        status,
        tip_type,
        amount,
        to,
        from,
        issue_id,
        contributor_id,
        curator_id,
        created_at,
        updated_at,
    }
}
