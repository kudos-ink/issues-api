use base64::encode;
use serde_derive::Deserialize;
use serde_json::from_slice;
use serde_json::{self, json, Value};
use std::convert::Infallible;
use std::env;
use warp::Filter;
use warp::{
    http::StatusCode,
    hyper::{body::to_bytes, Body, Response},
    test::request,
};

use crate::{
    types::ApiConfig,
    utils::{setup_db, setup_filters},
};

pub fn basic_auth_header(username: &str, password: &str) -> String {
    let credentials = format!("{}:{}", username, password);
    let encoded_credentials = encode(credentials);
    format!("Basic {}", encoded_credentials)
}

pub async fn setup_test_auth() -> String {
    let pseudo = env::var("PSEUDO").unwrap_or_default();
    let password = env::var("PASSWORD").unwrap_or_default();
    let auth_header = basic_auth_header(&pseudo, &password);
    auth_header
}

pub async fn setup_test_app() -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
    let db_access = setup_db(&ApiConfig::new().database_url).await;
    let app_filters = setup_filters(db_access.clone());
    app_filters
}

pub async fn send_request(
    app: &(impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone + Send + 'static),
    method: &str,
    path_with_query: &str,
    payload: Option<Vec<u8>>,
    auth_header: Option<&str>,
) -> Response<Body> {
    let mut req_builder = request().method(method).path(path_with_query);

    if let Some(body) = payload {
        req_builder = req_builder.body(body);
    }

    if let Some(header) = auth_header {
        req_builder = req_builder.header("Authorization", header);
    }

    let resp = req_builder.reply(app).await;

    Response::builder()
        .status(resp.status())
        .body(Body::from(resp.body().to_vec()))
        .expect("Failed to construct response object")
}

#[derive(Deserialize)]
struct IdResponse {
    pub id: i32,
}

async fn send_and_extract_id(
    app: &(impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone + Send + 'static),
    method: &str,
    path: &str,
    payload: Value,
) -> Result<i32, Infallible> {
    let pseudo = env::var("PSEUDO").unwrap_or_default();
    let password = env::var("PASSWORD").unwrap_or_default();
    let auth_header = basic_auth_header(&pseudo, &password);

    let resp = send_request(
        app,
        method,
        path,
        Some(json!(payload).to_string().into_bytes()),
        Some(&auth_header),
    )
    .await;

    assert_eq!(resp.status(), StatusCode::OK, "Failed to process request");

    let body_bytes = to_bytes(resp.into_body())
        .await
        .expect("Failed to read response body");
    let id_response: IdResponse = from_slice(&body_bytes).expect("Failed to deserialize response");

    Ok(id_response.id)
}

pub async fn create_organization(
    app: &(impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone + Send + 'static),
    name: &str,
) -> Result<i32, Infallible> {
    let payload = json!({ "name": name });
    send_and_extract_id(app, "POST", "/organizations", payload).await
}

// async fn create_repository(
//     app: impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone + Send + 'static,
//     organization_id: i32,
// ) -> Result<i32, Infallible> {
//     let payload = json!({ "name": "New Repository", "organization_id": organization_id });
//     send_and_extract_id(app, "POST", "/repositories", payload).await
// }

// async fn create_issue(
//     app: impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone + Send + 'static,
//     repository_id: i32,
// ) -> Result<i32, Infallible> {
//     let payload = json!({ "title": "New Issue", "repository_id": repository_id });
//     send_and_extract_id(app, "POST", "/issues", payload).await
// }

// async fn create_tip(
//     app: impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone + Send + 'static,
//     issue_id: i32,
// ) -> Result<i32, Infallible> {
//     let payload = json!({ "amount": 100, "issue_id": issue_id });
//     send_and_extract_id(app, "POST", "/tips", payload).await
// }
