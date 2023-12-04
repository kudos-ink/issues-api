use warp::{reply::{Reply, json}, reject::{Rejection}};

use super::{db::DBContribution, models::{ContributionRequest, ContributionResponse}};

pub async fn create_contribution_handler(
    body: ContributionRequest,
    db_access: impl DBContribution,
) -> Result<impl Reply, Rejection> {
    Ok(json(&ContributionResponse::of(
        db_access.create_contribution(body)
            .await?,
    )))
}
