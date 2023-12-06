use crate::{
    contributions::routes::routes,
    contributions::{
        db::DBContribution,
        models::{Contribution, ContributionRequest, ContributionResponse},
    }, handlers::{error_handler, ErrorResponse}
};
use mobc::async_trait;
use warp::{reject, test::request, Filter};


const EXISTING_ID: i64 = 1;
const NOT_FOUND_ID: i64 = 2;

#[derive(Clone)]
pub struct DBMockWithContributions {}

#[async_trait]
impl DBContribution for DBMockWithContributions {
    async fn get_contribution(&self, id: i64) -> Result<Option<Contribution>, reject::Rejection> {
        match id {
            EXISTING_ID => Ok(Some(Contribution { id })),
            NOT_FOUND_ID => Ok(None),
            _ => Ok(None)
        }
    }
    async fn get_contributions(&self) -> Result<Vec<Contribution>, reject::Rejection> {
        Ok(vec![Contribution { id: 1 }])
    }
    async fn create_contribution(
        &self,
        contribution: ContributionRequest,
    ) -> Result<Contribution, reject::Rejection> {
        Ok(Contribution {
            id: contribution.id,
        })
    }
    async fn delete_contribution(&self, _: i64) -> Result<(), reject::Rejection> {
        Ok(())
    }
}

#[tokio::test]
async fn test_get_contribution_mock_db() {
    let id = EXISTING_ID;
    let r = routes(DBMockWithContributions {});
    let resp = request()
        .path(&format!("/contribution/{id}"))
        .reply(&r)
        .await;
    assert_eq!(resp.status(), 200);
    let body = resp.into_body();
    assert!(!body.is_empty());
    let expected_response = ContributionResponse { id };
    let response: ContributionResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(response, expected_response)
}

#[tokio::test]
async fn test_get_contribution_not_found_mock_db() {
    let id = NOT_FOUND_ID;
    let r = routes(DBMockWithContributions {}).recover(error_handler);
    let resp = request()
        .path(&format!("/contribution/{id}"))
        .reply(&r)
        .await;
    
    assert_eq!(resp.status(), 404);
    let body = resp.into_body();
    assert!(!body.is_empty());
    
    let expected_response = ErrorResponse {message: "Contribution not found".to_string()};
    let response: ErrorResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(response, expected_response)
}
