#[cfg(test)]
pub mod tests {
    use crate::error_handler::{error_handler, ErrorResponse};
    use crate::organization::db::DBOrganization;
    use crate::organization::models::{Organization, OrganizationRequest};
    use crate::pagination::GetPagination;
    use crate::repository::models::{
        NewRepository, RepositoriesRelations, RepositoryResponse, RepositorySort,
    };
    use crate::repository::routes::routes;
    use crate::repository::{db::DBRepository, models::Repository};
    use mobc::async_trait;
    use warp::test::request;
    use warp::{reject, Filter};

    #[derive(Clone)]
    pub struct RepositoriesDBMock {}

    #[async_trait]
    impl DBRepository for RepositoriesDBMock {
        async fn get_repository(
            &self,
            id: i32,
            relations: RepositoriesRelations,
        ) -> Result<Option<Repository>, reject::Rejection> {
            if id == 1 {
                Ok(Some(Repository {
                    id,
                    name: "repo".to_owned(),
                    organization_id: 1,
                    icon: "icon".to_string(),
                    url: "url".to_string(),
                    e_tag: "e_tag".to_string(),
                }))
            } else {
                Ok(None)
            }
        }
        async fn get_repository_by_name(
            &self,
            name: &str,
            relations: RepositoriesRelations,
        ) -> Result<Option<Repository>, reject::Rejection> {
            if name == "not_found" || name == "new" {
                Ok(None)
            } else {
                Ok(Some(Repository {
                    id: 1,
                    name: "repo".to_owned(),
                    organization_id: 1,
                    icon: "icon".to_string(),
                    url: "url".to_string(),
                    e_tag: "e_tag".to_string(),
                }))
            }
        }
        async fn get_repositories(
            &self,
            relations: RepositoriesRelations,
            pagination: GetPagination,
            sort: RepositorySort,
        ) -> Result<Vec<Repository>, reject::Rejection> {
            Ok(vec![])
        }
        async fn create_repository(
            &self,
            repository: NewRepository,
        ) -> Result<Repository, reject::Rejection> {
            Ok(Repository {
                name: repository.name.to_string(),
                id: 1,
                organization_id: repository.organization_id,
                icon: repository.icon,
                url: repository.url,
                e_tag: repository.e_tag,
            })
        }
        async fn delete_repository(&self, id: i32) -> Result<(), reject::Rejection> {
            Ok(())
        }
    }

    #[async_trait]
    impl DBOrganization for RepositoriesDBMock {
        async fn get_organization(
            &self,
            id: i32,
        ) -> Result<Option<Organization>, reject::Rejection> {
            Ok(Some(Organization {
                id,
                name: "ok".to_string(),
            }))
        }
        async fn get_organization_by_name(
            &self,
            name: &str,
        ) -> Result<Option<Organization>, reject::Rejection> {
            Ok(Some(Organization {
                id: 1,
                name: name.to_string(),
            }))
        }
        async fn get_organizations(&self) -> Result<Vec<Organization>, reject::Rejection> {
            Ok(vec![
                (Organization {
                    id: 1,
                    name: "name".to_string(),
                }),
            ])
        }
        async fn create_organization(
            &self,
            organization: OrganizationRequest,
        ) -> Result<Organization, reject::Rejection> {
            Ok(Organization {
                id: 1,
                name: organization.name,
            })
        }
        async fn delete_organization(&self, id: i32) -> Result<(), reject::Rejection> {
            Ok(())
        }
    }
    #[tokio::test]
    async fn test_get_repo_by_id_not_found() {
        let id = 2;
        let r = routes(RepositoriesDBMock {}).recover(error_handler);
        let resp = request()
            .path(&format!("/repositories/{id}"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 404);
        let body = resp.into_body();
        assert!(!body.is_empty());

        let response: ErrorResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(
            response,
            ErrorResponse {
                message: format!("Repository #{id} not found",),
            }
        )
    }
    #[tokio::test]
    async fn test_get_repository_by_id_exists() {
        let id = 1;
        let r = routes(RepositoriesDBMock {}).recover(error_handler);
        let resp = request()
            .path(&format!("/repositories/{id}"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 200);
        let body = resp.into_body();
        let response: RepositoryResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            response,
            RepositoryResponse {
                id,
                name: "repo".to_owned(),
                organization_id: 1,
                icon: "icon".to_string(),
                url: "url".to_string(),
                e_tag: "e_tag".to_string(),
            }
        )
    }
    #[tokio::test]
    async fn test_get_repository_by_name_not_found() {
        let name = "not_found";
        let r = routes(RepositoriesDBMock {}).recover(error_handler);
        let resp = request()
            .path(&format!("/repositories/name/{name}"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 404);
        let body = resp.into_body();
        assert!(!body.is_empty());

        let response: ErrorResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            response,
            ErrorResponse {
                message: format!("Repository {name} not found",),
            }
        )
    }
    #[tokio::test]
    async fn test_get_repository_by_name_exists() {
        let name = "repo";
        let r = routes(RepositoriesDBMock {}).recover(error_handler);
        let resp = request()
            .path(&format!("/repositories/name/{name}"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 200);
        let body = resp.into_body();
        let expected_response = RepositoryResponse {
            id: 1,
            name: "repo".to_owned(),
            organization_id: 1,
            icon: "icon".to_string(),
            url: "url".to_string(),
            e_tag: "e_tag".to_string(),
        };
        let response: RepositoryResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(response, expected_response)
    }
    #[tokio::test]
    async fn test_get_repositories() {
        let r = routes(RepositoriesDBMock {}).recover(error_handler);
        let resp = request().path(&format!("/repositories")).reply(&r).await;
        assert_eq!(resp.status(), 200);
        let body = resp.into_body();
        let response: Vec<RepositoryResponse> = serde_json::from_slice(&body).unwrap();
        assert_eq!(response, vec![]);
    }
    #[tokio::test]
    async fn test_get_repositories_valid_query_params() {
        let r = routes(RepositoriesDBMock {}).recover(error_handler);
        let resp = request().path(&format!("/repositories?languages=false&tips=false&maintainers=false&issues=false&limit=1&offset=10&sort_by=id&descending=false")).reply(&r).await;
        assert_eq!(resp.status(), 200);
        let body = resp.into_body();
        let response: Vec<RepositoryResponse> = serde_json::from_slice(&body).unwrap();
        assert_eq!(response, vec![]);
    }
    #[tokio::test]
    async fn test_get_repositories_invalid_query_params() {
        let r = routes(RepositoriesDBMock {}).recover(error_handler);
        let resp = request()
            .path(&format!("/repositories?languages=fal1se"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 401); // TODO: check why it's a 401
        let resp = request()
            .path(&format!("/repositories?maintainers=fal1se"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 401); // TODO: check why it's a 401
        let resp = request()
            .path(&format!("/repositories?issues=123"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 401); // TODO: check why it's a 401
        let resp = request()
            .path(&format!("/repositories?tips=123"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 401);
        let resp = request()
            .path(&format!("/repositories?limit=asd"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 401);
        let resp = request()
            .path(&format!("/repositories?offset=asd"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 401);
        let resp = request()
            .path(&format!("/repositories?sort_by=invalid"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 400);
        let resp = request()
            .path(&format!("/repositories?descending=sadf"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 401); // TODO: check why it's a 401
    }

    #[tokio::test]
    async fn test_create_repository_ok() {
        let id = 1;
        let name = "new".to_string();
        let icon = "icon".to_string();
        let e_tag = "e_tag".to_string();
        let url = "url".to_string();

        let new_repository: Vec<u8> = serde_json::to_vec(&NewRepository {
            name: name.clone(),
            icon: icon.clone(),
            organization_id: 1,
            url: url.clone(),
            e_tag: e_tag.clone(),
        })
        .unwrap();
        let r = routes(RepositoriesDBMock {}).recover(error_handler);
        let resp = request()
            .body(new_repository)
            .path(&"/repositories")
            .header("Authorization", "Basic dGVzdDp0ZXN0") // test:test
            .method("POST")
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 201);
        let body = resp.into_body();
        assert!(!body.is_empty());

        let expected_response = RepositoryResponse {
            id,
            name,
            organization_id: 1,
            icon,
            url,
            e_tag,
        };
        let response: RepositoryResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(response, expected_response)
    }

    #[tokio::test]
    async fn test_create_repository_already_exists() {
        let name = "name".to_string();
        let icon = "icon".to_string();
        let e_tag = "e_tag".to_string();
        let url = "url".to_string();

        let new_repository: Vec<u8> = serde_json::to_vec(&NewRepository {
            name: name.clone(),
            icon: icon.clone(),
            organization_id: 1,
            url: url.clone(),
            e_tag: e_tag.clone(),
        })
        .unwrap();
        let r = routes(RepositoriesDBMock {}).recover(error_handler);
        let resp = request()
            .body(new_repository)
            .path(&"/repositories")
            .header("Authorization", "Basic dGVzdDp0ZXN0") // test:test
            .method("POST")
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 400);
        let body = resp.into_body();
        assert!(!body.is_empty());

        let response: ErrorResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            response,
            ErrorResponse {
                message: format!("Repository #1 already exists",),
            }
        )
    }

    #[tokio::test]
    async fn test_delete_repository() {
        let id = 1;
        let r = routes(RepositoriesDBMock {}).recover(error_handler);
        let resp = request()
            .path(&format!("/repositories/{id}"))
            .method("DELETE")
            .header("Authorization", "Basic dGVzdDp0ZXN0") // test:test
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 204);
        let body = resp.into_body();
        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn test_delete_repository_does_not_exist_mock_db() {
        let id = 4;

        let r = routes(RepositoriesDBMock {}).recover(error_handler);
        let resp = request()
            .path(&format!("/repositories/4"))
            .method("DELETE")
            .header("Authorization", "Basic dGVzdDp0ZXN0") // test:test
            .reply(&r)
            .await;

        assert_eq!(resp.status(), 404);
        let body = resp.into_body();
        assert!(!body.is_empty());

        let expected_response = ErrorResponse {
            message: format!("Repository #{id} not found"),
        };
        let response: ErrorResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(response, expected_response)
    }
}
