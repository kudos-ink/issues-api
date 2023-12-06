use std::fmt;

use serde_derive::Deserialize;
use thiserror::Error;


#[derive(Error, Debug, Deserialize, PartialEq)]
pub enum ContributionError {  //TODO: implement Reply
    ContributionExists(i64),
    ContributionNotFound(i64),
}


impl fmt::Display for ContributionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContributionError::ContributionExists(id) => write!(f, "Contribution #{} already exists", id),
            ContributionError::ContributionNotFound(id) => write!(f, "Contribution #{} not found", id),
        }
    }
}

impl warp::reject::Reject for ContributionError {}