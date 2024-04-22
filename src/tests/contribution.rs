// #[cfg(test)]
// mod tests {
//     use crate::{
//         handlers::{error_handler, ErrorResponse},
//         init_db,
//         user::routes::routes,
//         user::{
//             db::DBUser,
//             models::{NewUser, User, UserResponse},
//         },
//     };
//     use mobc::async_trait;
//     use warp::{reject, test::request, Filter};

//     #[derive(Clone)]
//     pub struct DBMockValues {}
//     #[derive(Clone)]
//     pub struct DBMockEmpty {}

//     #[async_trait]
//     impl DBUser for DBMockValues {
//         async fn get_user(&self, id: i32) -> Result<Option<User>, reject::Rejection> {
//             Ok(Some(User {
//                 id,
//                 username: "username".to_string(),
//             }))
//         }
//         async fn get_user_by_username(
//             &self,
//             username: &str,
//         ) -> Result<Option<User>, reject::Rejection> {
//             Ok(Some(User {
//                 id: 1,
//                 username: username.to_string(),
//             }))
//         }
//         async fn get_users(&self) -> Result<Vec<User>, reject::Rejection> {
//             Ok(vec![User {
//                 id: 1,
//                 username: "username".to_string(),
//             }])
//         }
//         async fn create_user(&self, user: NewUser) -> Result<User, reject::Rejection> {
//             Ok(User {
//                 id: 1,
//                 username: user.username,
//             })
//         }
//         async fn delete_user(&self, _: i32) -> Result<(), reject::Rejection> {
//             Ok(())
//         }
//     }
//     #[async_trait]
//     impl DBUser for DBMockEmpty {
//         async fn get_user(&self, _: i32) -> Result<Option<User>, reject::Rejection> {
//             Ok(None)
//         }
//         async fn get_users(&self) -> Result<Vec<User>, reject::Rejection> {
//             Ok(vec![])
//         }
//         async fn get_user_by_username(
//             &self,
//             _username: &str,
//         ) -> Result<Option<User>, reject::Rejection> {
//             Ok(None)
//         }
//         async fn create_user(&self, user: NewUser) -> Result<User, reject::Rejection> {
//             Ok(User {
//                 id: 1,
//                 username: user.username,
//             })
//         }
//         async fn delete_user(&self, _: i32) -> Result<(), reject::Rejection> {
//             Ok(())
//         }
//     }

//     #[tokio::test]
//     async fn test_get_user_mock_db() {
//         let id = 1;
//         let r = routes(DBMockValues {});
//         let resp = request().path(&format!("/users/{id}")).reply(&r).await;
//         assert_eq!(resp.status(), 200);
//         let body = resp.into_body();
//         assert!(!body.is_empty());
//         let expected_response = UserResponse {
//             id,
//             username: "username".to_string(),
//         };
//         let response: UserResponse = serde_json::from_slice(&body).unwrap();
//         assert_eq!(response, expected_response)
//     }
//     #[tokio::test]
//     async fn test_get_user_not_found_mock_db() {
//         let id = 1;
//         let r = routes(DBMockEmpty {}).recover(error_handler);
//         let resp = request().path(&format!("/users/{id}")).reply(&r).await;

//         assert_eq!(resp.status(), 404);
//         let body = resp.into_body();
//         assert!(!body.is_empty());

//         let expected_response = ErrorResponse {
//             message: format!("User #{} not found", id),
//         };
//         let response: ErrorResponse = serde_json::from_slice(&body).unwrap();

//         assert_eq!(response, expected_response)
//     }

//     #[tokio::test]
//     async fn test_get_users_mock_db() {
//         let r = routes(DBMockValues {});
//         let resp = request().path(&format!("/users")).reply(&r).await;

//         assert_eq!(resp.status(), 200);
//         let body = resp.into_body();
//         assert!(!body.is_empty());
//         let expected_response = vec![UserResponse {
//             id: 1,
//             username: "username".to_string(),
//         }];
//         let response: Vec<UserResponse> = serde_json::from_slice(&body).unwrap();
//         assert_eq!(response, expected_response)
//     }

//     #[tokio::test]
//     async fn test_get_users_empty_mock_db() {
//         let r = routes(DBMockEmpty {});
//         let resp = request().path(&format!("/users")).reply(&r).await;
//         assert_eq!(resp.status(), 200);

//         let body = resp.into_body();
//         let response: Vec<UserResponse> = serde_json::from_slice(&body).unwrap();
//         let expected_response: Vec<UserResponse> = vec![];
//         assert_eq!(response, expected_response);
//     }

//     #[tokio::test]
//     async fn test_create_user_mock_db() {
//         let id = 1;
//         let new_user = serde_json::to_vec(&UserResponse {
//             id: 1,
//             username: "username".to_string(),
//         })
//         .unwrap();
//         let r = routes(DBMockEmpty {});
//         let resp = request()
//             .body(new_user)
//             .path(&"/users")
//             .method("POST")
//             .reply(&r)
//             .await;
//         assert_eq!(resp.status(), 200);
//         let body = resp.into_body();
//         assert!(!body.is_empty());

//         let expected_response = UserResponse {
//             id,
//             username: "username".to_string(),
//         };
//         let response: UserResponse = serde_json::from_slice(&body).unwrap();
//         assert_eq!(response, expected_response)
//     }

//     #[tokio::test]
//     async fn test_create_user_already_exists_mock_db() {
//         let id = 1;
//         let new_user = serde_json::to_vec(&UserResponse {
//             id: 1,
//             username: "username".to_string(),
//         })
//         .unwrap();
//         let r = routes(DBMockValues {}).recover(error_handler);
//         let resp = request()
//             .body(new_user)
//             .path(&"/users")
//             .method("POST")
//             .reply(&r)
//             .await;
//         assert_eq!(resp.status(), 400);
//         let body = resp.into_body();
//         assert!(!body.is_empty());

//         let expected_response = ErrorResponse {
//             message: format!("User #{} already exists", id),
//         };

//         let response: ErrorResponse = serde_json::from_slice(&body).unwrap();
//         assert_eq!(response, expected_response)
//     }

//     #[tokio::test]
//     async fn test_delete_user_mock_db() {
//         let id = 1;
//         let r = routes(DBMockValues {});
//         let resp = request()
//             .path(&format!("/users/{id}"))
//             .method("DELETE")
//             .reply(&r)
//             .await;
//         assert_eq!(resp.status(), 200);
//         let body = resp.into_body();
//         assert!(body.is_empty());
//     }

//     #[tokio::test]
//     async fn test_delete_user_does_not_exist_mock_db() {
//         let id = 1;
//         let new_user = serde_json::to_vec(&UserResponse {
//             id: 1,
//             username: "username".to_string(),
//         })
//         .unwrap();
//         let r = routes(DBMockEmpty {}).recover(error_handler);
//         let resp = request()
//             .body(new_user)
//             .path(&format!("/users/{id}"))
//             .method("DELETE")
//             .reply(&r)
//             .await;

//         assert_eq!(resp.status(), 404);
//         let body = resp.into_body();
//         assert!(!body.is_empty());

//         let expected_response = ErrorResponse {
//             message: format!("User #{} not found", id),
//         };
//         let response: ErrorResponse = serde_json::from_slice(&body).unwrap();
//         assert_eq!(response, expected_response)
//     }

//     #[tokio::test]
//     #[ignore]
//     async fn test_create_user_db() {
//         let id = 1;
//         let new_user = serde_json::to_vec(&UserResponse {
//             id: 1,
//             username: "username".to_string(),
//         })
//         .unwrap();
//         let db = init_db(
//             "postgres://postgres:password@localhost:5432/database".to_string(),
//             "db_test.sql".to_string(),
//         )
//         .await
//         .unwrap();
//         let r = routes(db);
//         let resp = request()
//             .body(new_user)
//             .path(&"/users")
//             .method("POST")
//             .reply(&r)
//             .await;
//         assert_eq!(resp.status(), 200);
//         let body = resp.into_body();
//         assert!(!body.is_empty());

//         let expected_response = UserResponse {
//             id,
//             username: "username".to_string(),
//         };
//         let response: UserResponse = serde_json::from_slice(&body).unwrap();
//         assert_eq!(response, expected_response)
//     }

//     #[tokio::test]
//     #[ignore]
//     async fn test_create_user_already_exists_db() {
//         let id = 1;
//         let new_user = serde_json::to_vec(&UserResponse {
//             id: 1,
//             username: "username".to_string(),
//         })
//         .unwrap();
//         let db = init_db(
//             "postgres://postgres:password@localhost:5432/database".to_string(),
//             "db_test.sql".to_string(),
//         )
//         .await
//         .unwrap();
//         let r = routes(db).recover(error_handler);
//         let _ = request()
//             .body(new_user.clone())
//             .path(&"/users")
//             .method("POST")
//             .reply(&r)
//             .await;
//         let resp = request()
//             .body(new_user)
//             .path(&"/users")
//             .method("POST")
//             .reply(&r)
//             .await;
//         assert_eq!(resp.status(), 400);
//         let body = resp.into_body();
//         assert!(!body.is_empty());

//         let expected_response = ErrorResponse {
//             message: format!("User #{} already exists", id),
//         };

//         let response: ErrorResponse = serde_json::from_slice(&body).unwrap();
//         assert_eq!(response, expected_response)
//     }

//     #[tokio::test]
//     #[ignore]
//     async fn test_get_user_not_found_db() {
//         let id = 1;
//         let db = init_db(
//             "postgres://postgres:password@localhost:5432/database".to_string(),
//             "db_test.sql".to_string(),
//         )
//         .await
//         .unwrap();
//         let r = routes(db).recover(error_handler);
//         let resp = request().path(&format!("/users/{id}")).reply(&r).await;

//         assert_eq!(resp.status(), 404);
//         let body = resp.into_body();
//         assert!(!body.is_empty());

//         let expected_response = ErrorResponse {
//             message: "User #1 not found".to_string(),
//         };
//         let response: ErrorResponse = serde_json::from_slice(&body).unwrap();

//         assert_eq!(response, expected_response)
//     }

//     #[tokio::test]
//     #[ignore]
//     async fn test_get_user_db() {
//         let id = 1;
//         let new_user = serde_json::to_vec(&UserResponse {
//             id: 1,
//             username: "username".to_string(),
//         })
//         .unwrap();
//         let db = init_db(
//             "postgres://postgres:password@localhost:5432/database".to_string(),
//             "db_test.sql".to_string(),
//         )
//         .await
//         .unwrap();
//         let r = routes(db);
//         let _ = request()
//             .body(new_user)
//             .path(&"/users")
//             .method("POST")
//             .reply(&r)
//             .await;
//         let resp = request().path(&format!("/users/{id}")).reply(&r).await;
//         // assert_eq!(resp.status(), 200);
//         let body = resp.into_body();
//         assert!(!body.is_empty());
//         let expected_response = UserResponse {
//             id,
//             username: "username".to_string(),
//         };
//         let response: UserResponse = serde_json::from_slice(&body).unwrap();
//         assert_eq!(response, expected_response)
//     }

//     #[tokio::test]
//     #[ignore]
//     async fn test_get_users_empty_db() {
//         let db = init_db(
//             "postgres://postgres:password@localhost:5432/database".to_string(),
//             "db_test.sql".to_string(),
//         )
//         .await
//         .unwrap();
//         let r = routes(db).recover(error_handler);
//         let resp = request().path(&format!("/users")).reply(&r).await;

//         assert_eq!(resp.status(), 200);
//         let body = resp.into_body();
//         let response: Vec<UserResponse> = serde_json::from_slice(&body).unwrap();
//         let expected_response: Vec<UserResponse> = vec![];
//         assert_eq!(response, expected_response);
//     }
//     #[tokio::test]
//     #[ignore]
//     async fn test_get_users_db() {
//         let id = 1;
//         let new_user = serde_json::to_vec(&UserResponse {
//             id: 1,
//             username: "username".to_string(),
//         })
//         .unwrap();
//         let db = init_db(
//             "postgres://postgres:password@localhost:5432/database".to_string(),
//             "db_test.sql".to_string(),
//         )
//         .await
//         .unwrap();
//         let r = routes(db);
//         let _ = request()
//             .body(new_user)
//             .path(&"/users")
//             .method("POST")
//             .reply(&r)
//             .await;
//         let resp = request().path(&format!("/users")).reply(&r).await;
//         assert_eq!(resp.status(), 200);

//         let body = resp.into_body();
//         assert!(!body.is_empty());
//         let expected_response = vec![UserResponse {
//             id,
//             username: "username".to_string(),
//         }];
//         let response: Vec<UserResponse> = serde_json::from_slice(&body).unwrap();
//         assert_eq!(response, expected_response)
//     }

//     #[tokio::test]
//     #[ignore]
//     async fn test_delete_user_db() {
//         let id = 1;
//         let new_user = serde_json::to_vec(&UserResponse {
//             id: 1,
//             username: "username".to_string(),
//         })
//         .unwrap();
//         let db = init_db(
//             "postgres://postgres:password@localhost:5432/database".to_string(),
//             "db_test.sql".to_string(),
//         )
//         .await
//         .unwrap();
//         let r = routes(db).recover(error_handler);
//         let resp = request()
//             .body(new_user)
//             .path(&"/users")
//             .method("POST")
//             .reply(&r)
//             .await;
//         assert_eq!(resp.status(), 200);

//         let resp = request()
//             .path(&format!("/users/{id}"))
//             .method("DELETE")
//             .reply(&r)
//             .await;
//         assert_eq!(resp.status(), 200);
//         let body = resp.into_body();
//         assert!(body.is_empty());

//         let resp = request().path(&format!("/users/{id}")).reply(&r).await;

//         assert_eq!(resp.status(), 404);
//         let body = resp.into_body();
//         assert!(!body.is_empty());
//     }

//     #[tokio::test]
//     #[ignore]
//     async fn test_delete_user_does_not_exist_db() {
//         let id = 1;
//         let db = init_db(
//             "postgres://postgres:password@localhost:5432/database".to_string(),
//             "db_test.sql".to_string(),
//         )
//         .await
//         .unwrap();
//         let new_user = serde_json::to_vec(&UserResponse {
//             id: 1,
//             username: "username".to_string(),
//         })
//         .unwrap();
//         let r = routes(db).recover(error_handler);
//         let resp = request()
//             .body(new_user)
//             .path(&format!("/users/{id}"))
//             .method("DELETE")
//             .reply(&r)
//             .await;

//         assert_eq!(resp.status(), 404);
//         let body = resp.into_body();
//         assert!(!body.is_empty());

//         let expected_response = ErrorResponse {
//             message: "User #1 not found".to_string(),
//         };
//         let response: ErrorResponse = serde_json::from_slice(&body).unwrap();
//         assert_eq!(response, expected_response)
//     }
// }
