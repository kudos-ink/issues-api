use std::fmt;

use thiserror::Error;


#[derive(Error, Debug)]
pub enum ContributionError {
    ContributionExists(i64),
    ContributionNotFound(i64),
}


impl fmt::Display for ContributionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContributionError::ContributionExists(_) => write!(f, "Contribution already exists"),
            ContributionError::ContributionNotFound(_) => write!(f, "Contribution not found"),
        }
    }
}

impl warp::reject::Reject for ContributionError {}