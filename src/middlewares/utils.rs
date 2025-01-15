
use warp::
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use super::errors::AuthenticationError;

pub const BASIC: &str = "Basic ";
pub const BEARER: &str = "Bearer ";

pub fn token_from_header(headers: &HeaderMap<HeaderValue>, name: &str) -> Result<String, AuthenticationError> {
    let header = headers
        .get(AUTHORIZATION)
        .ok_or(AuthenticationError::NoAuthHeader)?;
    let auth_header = std::str::from_utf8(header.as_bytes())
        .map_err(|_| AuthenticationError::InvalidAuthHeader)?;

    if !auth_header.starts_with(name) {
        return Err(AuthenticationError::InvalidAuthHeader);
    }
    Ok(auth_header.trim_start_matches(name).to_owned())
}