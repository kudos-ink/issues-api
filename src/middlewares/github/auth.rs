use crate::middlewares::{
    errors::AuthenticationError,
    utils::{token_from_header, BEARER},
};
use reqwest::Client;

use warp::{
    http::header::{HeaderMap, HeaderValue},
    reject, Filter, Rejection,
};
use log::error;

use super::model::GitHubUser;


pub fn with_github_auth() -> impl Filter<Extract = (GitHubUser,), Error = Rejection> + Clone {
    warp::filters::header::headers_cloned()
        .and_then(authorize)
        // .untuple_one()
}
async fn authorize(headers: HeaderMap<HeaderValue>) -> Result<GitHubUser, Rejection> {
    match token_from_header(&headers, BEARER) {
        Ok(token) => {
            let client = Client::new();
            let response = client
                .get("https://api.github.com/user") // 5,000 requests per hour for auth'd requests
                .header("Authorization", format!("{BEARER} {token}"))
                .header("User-Agent", "MoreKudos")
                .send()
                .await
                .map_err(|_| AuthenticationError::GitHub)?;

                if response.status().is_success() {
                    let user_data: serde_json::Value = response
                        .json()
                        .await
                        .map_err(|_| reject::custom(AuthenticationError::GitHub))?;
    
                    // Extract user ID and username
                    let id = user_data
                        .get("id")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| reject::custom(AuthenticationError::GitHub))?;
                    let username = user_data
                        .get("login")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| reject::custom(AuthenticationError::GitHub))?
                        .to_string();
    
                    let avatar_url = user_data// user for new users
                        .get("avatar_url")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| reject::custom(AuthenticationError::GitHub))?
                        .to_string();
                    Ok(GitHubUser { id, username, avatar_url })
                } else {
                    let status = response.status();
                    let body = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unable to read body".to_string());
                    error!(
                        "GitHub token validation failed. Status: {:?}, Body: {}",
                        status, body
                    );
    
                    if status.is_client_error() {
                        Err(reject::custom(AuthenticationError::WrongCredentials))
                    } else {
                        Err(reject::custom(AuthenticationError::GitHub))
                    }
                }
        }
        Err(e) => Err(reject::custom(e)),
    }
}