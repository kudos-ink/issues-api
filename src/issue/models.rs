use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Issue {
    pub id: i32,
    pub repository_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct IssueCreateRequest {
    pub url: String,
}
#[derive(Serialize, Deserialize)]
pub struct IssueGetRequest {
    pub id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct IssueResponse {
    pub id: i32,
    pub repository_id: i32,
    // TODO: add tip
}

impl IssueResponse {
    pub fn of(issue: Issue) -> IssueResponse {
        IssueResponse {
            id: issue.id,
            repository_id: issue.repository_id,
        }
    }
}

pub struct IssueInfo {
    pub organization: String,
    pub repository: String,
    pub issue_id: u32,
}

pub struct IssueCreate {
    pub repository_id: u32,
    pub id: u32,
    pub url: String,
}
