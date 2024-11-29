use base64::Engine;
use std::env;
use warp::{
    http::header::{HeaderMap, HeaderValue},
    reject, Filter, Rejection,
};

use crate::middlewares::errors::AuthenticationError;
use crate::middlewares::utils::{BASIC,token_from_header};



pub fn with_basic_auth() -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::filters::header::headers_cloned()
        .and_then(authorize)
        .untuple_one()
}
async fn authorize(headers: HeaderMap<HeaderValue>) -> Result<(), Rejection> {
    match token_from_header(&headers, BASIC) {
        Ok(token) => {
            let credentials = base64::prelude::BASE64_STANDARD
                .decode(token)
                .map_err(|_| reject::custom(AuthenticationError::WrongCredentials))?;
            let credentials_str = String::from_utf8(credentials)
                .map_err(|_| reject::custom(AuthenticationError::WrongCredentials))?;
            let credentials: Vec<&str> = credentials_str.split(':').collect();

            if credentials.len() == 2 {
                let expected_username = env::var("USERNAME").expect("USERNAME is not set");
                let expected_password = env::var("PASSWORD").expect("PASSWORD is not set");
                if credentials[0] == expected_username && credentials[1] == expected_password {
                    Ok(())
                } else {
                    Err(reject::custom(AuthenticationError::WrongCredentials))
                }
            } else {
                Err(reject::custom(AuthenticationError::WrongCredentials))
            }
        }
        Err(e) => Err(reject::custom(e)),
    }
}

