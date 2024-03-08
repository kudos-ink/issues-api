use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, Reply},
};

use crate::types::{IssueId, RepositoryId, TipId};

use super::{
    db::DBTip,
    errors::TipError,
    models::{CreateTipRequest, TipResponse, UpdateTipRequest},
};

pub async fn get_tip_handler(id: TipId, db_access: impl DBTip) -> Result<impl Reply, Rejection> {
    match db_access.get_tip(id).await? {
        Some(tip) => Ok(json(&TipResponse::of(tip))),
        None => Err(warp::reject::custom(TipError::NotFound)),
    }
}

pub async fn get_tip_by_issue_handler(id: IssueId, db_access: impl DBTip) -> Result<impl Reply, Rejection> {
    match db_access.get_tip_by_issue(id).await? {
        Some(tip) => Ok(json(&TipResponse::of(tip))),
        None => Err(warp::reject::custom(TipError::NotFound)),
    }
}

pub async fn get_tips_by_repository_handler(id: RepositoryId, db_access: impl DBTip) -> Result<impl Reply, Rejection> {
    let tips = db_access.get_tips_by_repository(id).await?;
    Ok(warp::reply::json(&tips.into_iter().map(TipResponse::of).collect::<Vec<_>>()))
}

pub async fn create_tip_handler(
    body: CreateTipRequest,
    db_access: impl DBTip,
) -> Result<impl Reply, Rejection> {
    match db_access.get_tip_by_issue(body.issue_id).await? {
        Some(tip) => Err(warp::reject::custom(TipError::TipExists(tip.id)))?,
        //TODO: Upsert contributor/curator
        None => Ok(json(&TipResponse::of(db_access.create_tip(body).await?))),
    }
}

pub async fn update_tip_handler(id: TipId, body: UpdateTipRequest, db_access: impl DBTip) -> Result<impl Reply, Rejection> {
    body.validate().map_err(|err| warp::reject::custom(err))?;

    match db_access.update_tip(id, body).await? {
        Some(tip) => Ok(json(&TipResponse::of(tip))),
        None => Err(warp::reject::custom(TipError::NotFound)),
    }
}

pub async fn delete_tip_handler(id: TipId, db_access: impl DBTip) -> Result<impl Reply, Rejection> {
    db_access.delete_tip(id).await?;
    Ok(StatusCode::OK)
}