use crate::error::AuthenticationError;
use std::env;
use warp::{
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
};
const BASIC: &str = "Basic ";
pub fn with_auth() -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::filters::header::headers_cloned()
        .and_then(authorize)
        .untuple_one()
}
async fn authorize(headers: HeaderMap<HeaderValue>) -> Result<(), Rejection> {
    match token_from_header(&headers) {
        Ok(token) => {
            let credentials = base64::decode(token)
                .map_err(|_| reject::custom(AuthenticationError::WrongCredentialsError))?;
            let credentials_str = String::from_utf8(credentials)
                .map_err(|_| reject::custom(AuthenticationError::WrongCredentialsError))?;
            let credentials: Vec<&str> = credentials_str.split(':').collect();

            if credentials.len() == 2 {
                let expected_username = env::var("PSEUDO")
                    .map_err(|_| reject::custom(AuthenticationError::WrongCredentialsError))?;
                let expected_password = env::var("PASSWORD")
                    .map_err(|_| reject::custom(AuthenticationError::WrongCredentialsError))?;

                if credentials[0] == expected_username && credentials[1] == expected_password {
                    Ok(())
                } else {
                    Err(reject::custom(AuthenticationError::WrongCredentialsError))
                }
            } else {
                Err(reject::custom(AuthenticationError::WrongCredentialsError))
            }
        }
        Err(e) => Err(reject::custom(e)),
    }
}

fn token_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String, AuthenticationError> {
    let header = headers
        .get(AUTHORIZATION)
        .ok_or(AuthenticationError::NoAuthHeaderError)?;
    let auth_header = std::str::from_utf8(header.as_bytes())
        .map_err(|_| AuthenticationError::InvalidAuthHeaderError)?;

    if !auth_header.starts_with(BASIC) {
        return Err(AuthenticationError::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BASIC).to_owned())
}
