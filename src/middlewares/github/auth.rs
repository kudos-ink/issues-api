use crate::middlewares::{
    errors::AuthenticationError,
    utils::{token_from_header, BEARER},
};
use surf;
use surf::http::headers::{AUTHORIZATION, USER_AGENT};

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
            let mut response = surf::get("https://api.github.com/user")
                .header(AUTHORIZATION, format!("{BEARER} {token}"))
                .header(USER_AGENT, "MoreKudos")
                .await
                .map_err(|e| {
                    error!("Error calling GitHub API: {e}");
                    AuthenticationError::GitHub
                })?;
    
            if response.status().is_success() {
                let user_data: serde_json::Value = response
                    .body_json()
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
                let email = user_data
                    .get("email")
                    .and_then(|v| v.as_str().map(|s| s.to_string()));
    
                let avatar_url = user_data
                    .get("avatar_url")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| reject::custom(AuthenticationError::GitHub))?
                    .to_string();
    
                Ok(GitHubUser { id, username, avatar_url, email })
            } else {
                let status = response.status();
                let body = response
                    .body_string()
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



pub fn with_optional_github_auth() -> impl Filter<Extract = (Option<GitHubUser>,), Error = Rejection> + Clone {
    warp::filters::header::headers_cloned()
        .and_then(optional_authorize)
}


async fn optional_authorize(headers: HeaderMap<HeaderValue>) -> Result<Option<GitHubUser>, Rejection> {
    match token_from_header(&headers, BEARER) {
        Ok(token) => {
            let mut response = surf::get("https://api.github.com/user")
                .header(AUTHORIZATION, format!("{BEARER} {token}"))
                .header(USER_AGENT, "MoreKudos")
                .await
                .map_err(|e| { error!("Error calling GitHub API: {e}"); AuthenticationError::GitHub })?;

            if response.status().is_success() {
                let user_data: serde_json::Value = response.body_json().await.map_err(|_| reject::custom(AuthenticationError::GitHub))?;
                let id = user_data.get("id").and_then(|v| v.as_i64()).ok_or_else(|| reject::custom(AuthenticationError::GitHub))?;
                let username = user_data.get("login").and_then(|v| v.as_str()).ok_or_else(|| reject::custom(AuthenticationError::GitHub))?.to_string();
                let email = user_data.get("email").and_then(|v| v.as_str().map(|s| s.to_string()));
                let avatar_url = user_data.get("avatar_url").and_then(|v| v.as_str()).ok_or_else(|| reject::custom(AuthenticationError::GitHub))?.to_string();
                
                Ok(Some(GitHubUser { id, username, avatar_url, email }))
            } else {
                error!("Invalid GitHub token provided.");
                Err(reject::custom(AuthenticationError::WrongCredentials))
            }
        },
        Err(AuthenticationError::NoAuthHeader) | Err(AuthenticationError::InvalidAuthHeader) => {
            Ok(None)
        },
        Err(e) => Err(reject::custom(e)),
    }
}
