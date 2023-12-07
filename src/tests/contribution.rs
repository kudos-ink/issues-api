#[cfg(test)]
mod tests {
    use crate::{
        contributions::routes::routes,
        contributions::{
            db::DBContribution,
            models::{Contribution, ContributionRequest, ContributionResponse},
        },
        handlers::{error_handler, ErrorResponse},
        init_db,
    };
    use mobc::async_trait;
    use warp::{reject, test::request, Filter};

    #[derive(Clone)]
    pub struct DBMockValues {}
    #[derive(Clone)]
    pub struct DBMockEmpty {}

    #[async_trait]
    impl DBContribution for DBMockValues {
        async fn get_contribution(
            &self,
            id: i64,
        ) -> Result<Option<Contribution>, reject::Rejection> {
            Ok(Some(Contribution { id }))
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
    #[async_trait]
    impl DBContribution for DBMockEmpty {
        async fn get_contribution(
            &self,
            _: i64,
        ) -> Result<Option<Contribution>, reject::Rejection> {
            Ok(None)
        }
        async fn get_contributions(&self) -> Result<Vec<Contribution>, reject::Rejection> {
            Ok(vec![])
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
        let id = 1;
        let r = routes(DBMockValues {});
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
        let id = 1;
        let r = routes(DBMockEmpty {}).recover(error_handler);
        let resp = request()
            .path(&format!("/contribution/{id}"))
            .reply(&r)
            .await;

        assert_eq!(resp.status(), 404);
        let body = resp.into_body();
        assert!(!body.is_empty());

        let expected_response = ErrorResponse {
            message: "Contribution not found".to_string(),
        };
        let response: ErrorResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(response, expected_response)
    }

    #[tokio::test]
    async fn test_get_contributions_mock_db() {
        let r = routes(DBMockValues {});
        let resp = request().path(&format!("/contribution")).reply(&r).await;

        assert_eq!(resp.status(), 200);
        let body = resp.into_body();
        assert!(!body.is_empty());
        let expected_response = vec![ContributionResponse { id: 1 }];
        let response: Vec<ContributionResponse> = serde_json::from_slice(&body).unwrap();
        assert_eq!(response, expected_response)
    }

    #[tokio::test]
    async fn test_get_contributions_empty_mock_db() {
        let r = routes(DBMockEmpty {});
        let resp = request().path(&format!("/contribution")).reply(&r).await;
        assert_eq!(resp.status(), 200);
        
        let body = resp.into_body();
        let response: Vec<ContributionResponse> = serde_json::from_slice(&body).unwrap();
        let expected_response: Vec<ContributionResponse> = vec![];
        assert_eq!(response, expected_response);
    }

    #[tokio::test]
    async fn test_create_contribution_mock_db() {
        let id = 1;
        let new_contribution = serde_json::to_vec(&ContributionRequest { id }).unwrap();
        let r = routes(DBMockEmpty {});
        let resp = request()
            .body(new_contribution)
            .path(&"/contribution")
            .method("POST")
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
    async fn test_create_contribution_already_exists_mock_db() {
        let id = 1;
        let new_contribution = serde_json::to_vec(&ContributionRequest { id }).unwrap();
        let r = routes(DBMockValues {}).recover(error_handler);
        let resp = request()
            .body(new_contribution)
            .path(&"/contribution")
            .method("POST")
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 400);
        let body = resp.into_body();
        assert!(!body.is_empty());

        let expected_response = ErrorResponse {
            message: "Contribution already exists".to_string(),
        };

        let response: ErrorResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(response, expected_response)
    }

    #[tokio::test]
    async fn test_delete_contribution_mock_db() {
        let id = 1;
        let r = routes(DBMockValues {});
        let resp = request()
            .path(&format!("/contribution/{id}"))
            .method("DELETE")
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 200);
        let body = resp.into_body();
        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn test_delete_contribution_does_not_exist_mock_db() {
        let id = 1;
        let new_contribution = serde_json::to_vec(&ContributionRequest { id }).unwrap();
        let r = routes(DBMockEmpty {}).recover(error_handler);
        let resp = request()
            .body(new_contribution)
            .path(&format!("/contribution/{id}"))
            .method("DELETE")
            .reply(&r)
            .await;

        assert_eq!(resp.status(), 404);
        let body = resp.into_body();
        assert!(!body.is_empty());

        let expected_response = ErrorResponse {
            message: "Contribution not found".to_string(),
        };
        let response: ErrorResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(response, expected_response)
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_contribution_db() {
        let id = 1;
        let new_contribution = serde_json::to_vec(&ContributionRequest { id }).unwrap();
        let db = init_db(
            "postgres://postgres:password@localhost:5432/database".to_string(),
            "db_test.sql".to_string(),
        )
        .await;
        let r = routes(db);
        let resp = request()
            .body(new_contribution)
            .path(&"/contribution")
            .method("POST")
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
    #[ignore]
    async fn test_create_contribution_already_exists_db() {
        let id = 1;
        let new_contribution = serde_json::to_vec(&ContributionRequest { id }).unwrap();
        let db = init_db(
            "postgres://postgres:password@localhost:5432/database".to_string(),
            "db_test.sql".to_string(),
        )
        .await;
        let r = routes(db).recover(error_handler);
        let _ = request()
            .body(new_contribution.clone())
            .path(&"/contribution")
            .method("POST")
            .reply(&r)
            .await;
        let resp = request()
            .body(new_contribution)
            .path(&"/contribution")
            .method("POST")
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 400);
        let body = resp.into_body();
        assert!(!body.is_empty());

        let expected_response = ErrorResponse {
            message: "Contribution already exists".to_string(),
        };

        let response: ErrorResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(response, expected_response)
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_contribution_not_found_db() {
        let id = 1;
        let db = init_db(
            "postgres://postgres:password@localhost:5432/database".to_string(),
            "db_test.sql".to_string(),
        )
        .await;
        let r = routes(db).recover(error_handler);
        let resp = request()
            .path(&format!("/contribution/{id}"))
            .reply(&r)
            .await;

        assert_eq!(resp.status(), 404);
        let body = resp.into_body();
        assert!(!body.is_empty());

        let expected_response = ErrorResponse {
            message: "Contribution not found".to_string(),
        };
        let response: ErrorResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(response, expected_response)
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_contribution_db() {
        let id = 1;
        let new_contribution = serde_json::to_vec(&ContributionRequest { id }).unwrap();
        let db = init_db(
            "postgres://postgres:password@localhost:5432/database".to_string(),
            "db_test.sql".to_string(),
        )
        .await;
        let r = routes(db);
        let _ = request()
            .body(new_contribution)
            .path(&"/contribution")
            .method("POST")
            .reply(&r)
            .await;
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
    #[ignore]
    async fn test_get_contributions_empty_db() {
        let db = init_db(
            "postgres://postgres:password@localhost:5432/database".to_string(),
            "db_test.sql".to_string(),
        )
        .await;
        let r = routes(db).recover(error_handler);
        let resp = request().path(&format!("/contribution")).reply(&r).await;

        assert_eq!(resp.status(), 200);
        let body = resp.into_body();
        let response: Vec<ContributionResponse> = serde_json::from_slice(&body).unwrap();
        let expected_response: Vec<ContributionResponse> = vec![];
        assert_eq!(response, expected_response);
    }
    #[tokio::test]
    #[ignore]
    async fn test_get_contributions_db() {
        let id = 1;
        let new_contribution = serde_json::to_vec(&ContributionRequest { id }).unwrap();
        let db = init_db(
            "postgres://postgres:password@localhost:5432/database".to_string(),
            "db_test.sql".to_string(),
        )
        .await;
        let r = routes(db);
        let _ = request()
            .body(new_contribution)
            .path(&"/contribution")
            .method("POST")
            .reply(&r)
            .await;
        let resp = request().path(&format!("/contribution")).reply(&r).await;
        assert_eq!(resp.status(), 200);

        let body = resp.into_body();
        assert!(!body.is_empty());
        let expected_response = vec![ContributionResponse { id: 1 }];
        let response: Vec<ContributionResponse> = serde_json::from_slice(&body).unwrap();
        assert_eq!(response, expected_response)
    }

    #[tokio::test]
    #[ignore]
    async fn test_delete_contribution_db() {
        let id = 1;
        let new_contribution = serde_json::to_vec(&ContributionRequest { id }).unwrap();
        let db = init_db(
            "postgres://postgres:password@localhost:5432/database".to_string(),
            "db_test.sql".to_string(),
        )
        .await;
        let r = routes(db).recover(error_handler);
        let resp = request()
            .body(new_contribution)
            .path(&"/contribution")
            .method("POST")
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 200);

        let resp = request()
            .path(&format!("/contribution/{id}"))
            .method("DELETE")
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 200);
        let body = resp.into_body();
        assert!(body.is_empty());

        let resp = request()
            .path(&format!("/contribution/{id}"))
            .reply(&r)
            .await;

        assert_eq!(resp.status(), 404);
        let body = resp.into_body();
        assert!(!body.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn test_delete_contribution_does_not_exist_db() {
        let id = 1;
        let db = init_db(
            "postgres://postgres:password@localhost:5432/database".to_string(),
            "db_test.sql".to_string(),
        )
        .await;
        let new_contribution = serde_json::to_vec(&ContributionRequest { id }).unwrap();
        let r = routes(db).recover(error_handler);
        let resp = request()
            .body(new_contribution)
            .path(&format!("/contribution/{id}"))
            .method("DELETE")
            .reply(&r)
            .await;

        assert_eq!(resp.status(), 404);
        let body = resp.into_body();
        assert!(!body.is_empty());

        let expected_response = ErrorResponse {
            message: "Contribution not found".to_string(),
        };
        let response: ErrorResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(response, expected_response)
    }
}
