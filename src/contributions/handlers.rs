use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, Reply},
};

use super::{
    db::DBContribution,
    errors::ContributionError,
    models::{ContributionRequest, ContributionResponse},
};

pub async fn create_contribution_handler(
    body: ContributionRequest,
    db_access: impl DBContribution,
) -> Result<impl Reply, Rejection> {
    match db_access.get_contribution(body.id).await? {
        Some(_) => Err(ContributionError::ContributionExists(body.id))?,
        None => Ok(json(&ContributionResponse::of(
            db_access.create_contribution(body).await?,
        ))),
    }
}

pub async fn get_contribution_handler(
    id: i64,
    db_access: impl DBContribution,
) -> Result<impl Reply, Rejection> {
    match db_access.get_contribution(id).await? {
        None => Err(ContributionError::ContributionNotFound(id))?,
        Some(contribution) => Ok(json(&ContributionResponse::of(contribution))),
    }
}

pub async fn get_contributions_handler(
    db_access: impl DBContribution,
) -> Result<impl Reply, Rejection> {
    let contributions = db_access.get_contributions().await?;
    Ok(json::<Vec<_>>(
        &contributions
            .into_iter()
            .map(|t| ContributionResponse::of(t))
            .collect(),
    ))
}

pub async fn delete_contribution_handler(
    id: i64,
    db_access: impl DBContribution,
) -> Result<impl Reply, Rejection> {
    match db_access.get_contribution(id).await? {
        Some(_) => {
            let _ = &db_access.delete_contribution(id).await?;
            Ok(StatusCode::OK)
        }
        None => Err(ContributionError::ContributionNotFound(id))?,
    }
}
