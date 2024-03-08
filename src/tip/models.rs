use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};
use serde_derive::{Deserialize, Serialize};
use warp::reject::Reject;

use crate::types::{IssueId, TipId, UserId};

#[derive(Serialize, Deserialize, Clone, Debug, ToSql, FromSql)]
pub enum TipStatus {
    TipSet,
    TipPaid,
    TipRejected,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSql, FromSql)]
pub enum TipType {
    TipDirect,
    TipGov,
}

#[derive(Deserialize)]
pub struct Tip {
    pub id: TipId,
    pub status: TipStatus,
    pub tip_type: TipType,
    pub amount: u128,
    pub to: [u8; 32],
    pub from: [u8; 32],
    pub issue_id: IssueId,
    pub contributor_id: UserId,
    pub curator_id: UserId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTipRequest {
    pub tip_type: TipType,
    pub amount: u128,
    pub issue_id: IssueId,
    pub contributor_id: UserId,
    pub curator_id: UserId,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTipRequest {
    pub status: Option<TipStatus>,
    pub tip_type: Option<TipType>,
    pub amount: Option<u128>,
    pub to: Option<[u8; 32]>,
    pub from: Option<[u8; 32]>,
}

#[derive(Serialize, Deserialize)]
pub struct TipResponse {
    pub id: TipId,
    pub status: TipStatus,
    pub tip_type: TipType,
    pub amount: u128,
    pub to: [u8; 32],
    pub from: [u8; 32],
    pub issue_id: IssueId,
    pub contributor_id: UserId,
    pub curator_id: UserId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl TipResponse {
    pub fn of(tip: Tip) -> TipResponse {
        TipResponse {
            id: tip.id,
            status: tip.status,
            tip_type: tip.tip_type,
            amount: tip.amount,
            to: tip.to,
            from: tip.from,
            issue_id: tip.issue_id,
            contributor_id: tip.contributor_id,
            curator_id: tip.curator_id,
            created_at: tip.created_at,
            updated_at: tip.updated_at
        }
    }
}

#[derive(Debug)]
pub enum TipValidationError {
    InvalidAmount,
    MissingTipType,
    MissingToAddress,
    MissingFromAddress,
}

impl Reject for TipValidationError {}

impl std::fmt::Display for TipValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TipValidationError::InvalidAmount => write!(f, "Amount must be greater than zero."),
            TipValidationError::MissingTipType => write!(
                f,
                "Tip type must be specified when setting tip status to TipSet."
            ),
            TipValidationError::MissingToAddress => {
                write!(f, "'to' address must be specified when status is TipPaid.")
            }
            TipValidationError::MissingFromAddress => write!(
                f,
                "'from' address must be specified when status is TipPaid."
            ),
        }
    }
}

impl UpdateTipRequest {
    pub fn validate(&self) -> Result<(), TipValidationError> {
        if let Some(amount) = self.amount {
            if amount == 0 {
                return Err(TipValidationError::InvalidAmount);
            }
        }

        match self.status {
            Some(TipStatus::TipSet) if self.tip_type.is_none() => {
                Err(TipValidationError::MissingTipType)
            }
            Some(TipStatus::TipPaid) => {
                if self.to.is_none() {
                    return Err(TipValidationError::MissingToAddress);
                }
                if self.from.is_none() {
                    return Err(TipValidationError::MissingFromAddress);
                }
                Ok(())
            }
            _ => Ok(())
        }
    }
}
