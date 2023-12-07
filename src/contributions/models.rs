use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Contribution {
    pub id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ContributionRequest {
    pub id: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ContributionResponse {
    pub id: i64,
}

impl ContributionResponse {
    pub fn of(contribution: Contribution) -> ContributionResponse {
        ContributionResponse {
            id: contribution.id,
        }
    }
}
