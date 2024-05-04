#[cfg(test)]
mod tests {
    use crate::{
        errors::{self, ErrorResponse},
        pagination::GetPagination,
        repository::{
            db::DBRepository,
            models::{NewRepository, RepositoriesRelations, Repository, RepositorySort},
        },
        user::{
            db::DBUser,
            models::{NewUser, PatchUser, User, UserResponse, UserSort, UsersRelations},
            routes::routes,
        },
    };
    use mobc::async_trait;
    use warp::{reject, test::request, Filter};

    #[derive(Clone)]
    pub struct UsersDBMock {}

    #[async_trait]
    impl DBUser for UsersDBMock {
        async fn get_user(
            &self,
            id: i32,
            _: UsersRelations,
        ) -> Result<Option<User>, reject::Rejection> {
            if id == 1 {
                Ok(Some(User {
                    id: 1,
                    username: "username".to_string(),
                }))
            } else {
                Ok(None)
            }
        }
        async fn get_users(
            &self,
            _: UsersRelations,
            _: GetPagination,
            _: UserSort,
        ) -> Result<Vec<User>, reject::Rejection> {
            Ok(vec![])
        }
        async fn get_user_by_username(
            &self,
            username: &str,
            _: UsersRelations,
        ) -> Result<Option<User>, reject::Rejection> {
            if username == "username" {
                Ok(Some(User {
                    id: 1,
                    username: username.to_string(),
                }))
            } else {
                Ok(None)
            }
        }
        async fn create_user(&self, user: NewUser) -> Result<User, reject::Rejection> {
            Ok(User {
                id: 1,
                username: user.username,
            })
        }
        async fn delete_user(&self, _: i32) -> Result<(), reject::Rejection> {
            Ok(())
        }
        async fn update_user_maintainers(
            &self,
            _: i32,
            _: PatchUser,
        ) -> Result<User, reject::Rejection> {
            Ok(User {
                id: 1,
                username: "username".to_string(),
            })
        }
    }

    #[async_trait]
    impl DBRepository for UsersDBMock {
        async fn get_repository(
            &self,
            _: i32,
            _: RepositoriesRelations,
        ) -> Result<Option<Repository>, reject::Rejection> {
            Ok(None)
        }
        async fn get_repository_by_name(
            &self,
            _: &str,
            _: RepositoriesRelations,
        ) -> Result<Option<Repository>, reject::Rejection> {
            Ok(None)
        }
        async fn get_repositories(
            &self,
            _: RepositoriesRelations,
            _: GetPagination,
            _: RepositorySort,
        ) -> Result<Vec<Repository>, reject::Rejection> {
            Ok(vec![])
        }
        async fn create_repository(
            &self,
            _: NewRepository,
        ) -> Result<Repository, reject::Rejection> {
            Ok(Repository {
                id: 1,
                name: "repo".to_owned(),
                organization_id: 1,
                icon: "icon".to_string(),
                url: "url".to_string(),
                e_tag: "e_tag".to_string(),
            })
        }
        async fn delete_repository(&self, _: i32) -> Result<(), reject::Rejection> {
            Ok(())
        }
    }
    #[tokio::test]
    async fn test_get_user_by_id_not_found() {
        let id = 2;
        let r = routes(UsersDBMock {}).recover(errors::error_handler);
        let resp = request().path(&format!("/users/{id}")).reply(&r).await;
        assert_eq!(resp.status(), 404);
        let body = resp.into_body();
        assert!(!body.is_empty());

        let response: ErrorResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(
            response,
            ErrorResponse {
                message: format!("User #{id} not found",),
            }
        )
    }
    #[tokio::test]
    async fn test_get_user_by_id_exists() {
        let id = 1;
        let r = routes(UsersDBMock {}).recover(errors::error_handler);
        let resp = request().path(&format!("/users/{id}")).reply(&r).await;
        assert_eq!(resp.status(), 200);
        let body = resp.into_body();
        let response: UserResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            response,
            UserResponse {
                id,
                username: "username".to_string(),
            }
        )
    }
    #[tokio::test]
    async fn test_get_user_by_name_not_found() {
        let name = "not_found";
        let r = routes(UsersDBMock {}).recover(errors::error_handler);
        let resp = request()
            .path(&format!("/users/username/{name}"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 404);
        let body = resp.into_body();
        assert!(!body.is_empty());

        let response: ErrorResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            response,
            ErrorResponse {
                message: format!("User {name} not found",),
            }
        )
    }
    #[tokio::test]
    async fn test_get_user_by_name_exists() {
        let name = "username";
        let r = routes(UsersDBMock {}).recover(errors::error_handler);
        let resp = request()
            .path(&format!("/users/username/{name}"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 200);
        let body = resp.into_body();
        let expected_response = UserResponse {
            id: 1,
            username: name.to_string(),
        };
        let response: UserResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(response, expected_response)
    }
    #[tokio::test]
    async fn test_get_users() {
        let r = routes(UsersDBMock {}).recover(errors::error_handler);
        let resp = request().path(&format!("/users")).reply(&r).await;
        assert_eq!(resp.status(), 200);
        let body = resp.into_body();
        let response: Vec<UserResponse> = serde_json::from_slice(&body).unwrap();
        assert_eq!(response, vec![]);
    }
    #[tokio::test]
    async fn test_get_users_valid_query_params() {
        let r = routes(UsersDBMock {}).recover(errors::error_handler);
        let resp = request().path(&format!("/users?wishes=false&tips=false&maintainers=false&issues=false&limit=1&offset=10&sort_by=id&descending=false")).reply(&r).await;
        assert_eq!(resp.status(), 200);
        let body = resp.into_body();
        let response: Vec<UserResponse> = serde_json::from_slice(&body).unwrap();
        assert_eq!(response, vec![]);
    }
    #[tokio::test]
    async fn test_get_users_invalid_query_params() {
        let r = routes(UsersDBMock {}).recover(errors::error_handler);
        let resp = request()
            .path(&format!("/users?wishes=fal1se"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 401); // TODO: check why it's a 401
        let resp = request()
            .path(&format!("/users?maintainers=fal1se"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 401); // TODO: check why it's a 401
        let resp = request()
            .path(&format!("/users?issues=123"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 401); // TODO: check why it's a 401
        let resp = request().path(&format!("/users?tips=123")).reply(&r).await;
        assert_eq!(resp.status(), 401);
        let resp = request().path(&format!("/users?limit=asd")).reply(&r).await;
        assert_eq!(resp.status(), 401);
        let resp = request()
            .path(&format!("/users?offset=asd"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 401);
        let resp = request()
            .path(&format!("/users?sort_by=invalid"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 400);
        let resp = request()
            .path(&format!("/users?descending=sadf"))
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 401);
    }

    #[tokio::test]
    async fn test_create_user_ok() {
        let id = 1;
        let username = "new".to_string();
        let new_user: Vec<u8> = serde_json::to_vec(&NewUser {
            username: username.clone(),
            repositories: None,
        })
        .unwrap();
        let r = routes(UsersDBMock {}).recover(errors::error_handler);
        let resp = request()
            .body(new_user)
            .path(&"/users")
            .header("Authorization", "Basic dGVzdDp0ZXN0") // test:test
            .method("POST")
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 201);
        let body = resp.into_body();
        assert!(!body.is_empty());

        let expected_response = UserResponse { id, username };
        let response: UserResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(response, expected_response)
    }

    #[tokio::test]
    async fn test_patch_user_ok() {
        let new_user: Vec<u8> = serde_json::to_vec(&PatchUser {
            repositories: vec![1],
        })
        .unwrap();
        let r = routes(UsersDBMock {}).recover(errors::error_handler);
        let resp = request()
            .body(new_user)
            .path(&"/users/1/maintainers")
            .header("Authorization", "Basic dGVzdDp0ZXN0") // test:test
            .method("PATCH")
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 422);
        let body = resp.into_body();
        assert!(!body.is_empty());

        let response: ErrorResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            response,
            ErrorResponse {
                message: format!("User #1 cannot be updated: repository 1 does not exist",),
            }
        )
    }
    #[tokio::test]
    async fn test_create_user_already_exists() {
        let username = "username".to_string();
        let new_user: Vec<u8> = serde_json::to_vec(&NewUser {
            username: username.clone(),
            repositories: None,
        })
        .unwrap();
        let r = routes(UsersDBMock {}).recover(errors::error_handler);
        let resp = request()
            .body(new_user)
            .path(&"/users")
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
                message: format!("User #1 already exists",),
            }
        )
    }

    #[tokio::test]
    async fn test_delete_user() {
        let id = 1;
        let r = routes(UsersDBMock {}).recover(errors::error_handler);
        let resp = request()
            .path(&format!("/users/{id}"))
            .method("DELETE")
            .header("Authorization", "Basic dGVzdDp0ZXN0") // test:test
            .reply(&r)
            .await;
        assert_eq!(resp.status(), 204);
        let body = resp.into_body();
        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn test_delete_user_does_not_exist_mock_db() {
        let id = 4;

        let r = routes(UsersDBMock {}).recover(errors::error_handler);
        let resp = request()
            .path(&format!("/users/4"))
            .method("DELETE")
            .header("Authorization", "Basic dGVzdDp0ZXN0") // test:test
            .reply(&r)
            .await;

        assert_eq!(resp.status(), 404);
        let body = resp.into_body();
        assert!(!body.is_empty());

        let expected_response = ErrorResponse {
            message: format!("User #{id} not found"),
        };
        let response: ErrorResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(response, expected_response)
    }
}
