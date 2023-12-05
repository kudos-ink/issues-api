
use std::convert::Infallible;
use serde::Serialize;
use warp::{hyper::StatusCode, Rejection, Reply};

use crate::{db::errors::DBError, contributions::errors::ContributionError};

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

pub async fn error_handler(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if err
        .find::<warp::filters::body::BodyDeserializeError>()
        .is_some()
    {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(e) = err.find::<DBError>() {
        eprintln!("{}", e);
        match e {
            DBError::DBQuery(_) => {
                code = StatusCode::BAD_REQUEST;
                message = "Could not Execute request";
            }
            _ => {
                eprintln!("unhandled application error: {:?}", err);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error";
            }
        }
    } else if let Some(e) = err.find::<ContributionError>() {
        eprintln!("{}", e);
        match e {
            ContributionError::ContributionExists(_) => {
                code = StatusCode::BAD_REQUEST;
                message = "Contribution already exists";
            },
            ContributionError::ContributionNotFound(_) => {
                code = StatusCode::NOT_FOUND;
                message = "Contribution not found";
            },
            _ => {
                eprintln!("unhandled application error: {:?}", err);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error";
            }
        }
    }else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        eprintln!("unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
