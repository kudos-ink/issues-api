use warp::{
    reject::Rejection,
    reply::{json, Reply},
};

use super::{
    db::DBContribution,
    models::{ContributionRequest, ContributionResponse}, errors::ContributionError,
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
