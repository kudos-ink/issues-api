use super::{errors::IssueError, models::IssueInfo};
use url::Url;

pub fn parse_github_issue_url(url_str: &str) -> Result<IssueInfo, IssueError> {
    let url = Url::parse(url_str).map_err(|_| IssueError::IssueInvalidURL)?;
    let path_segments: Vec<&str> = url
        .path_segments()
        .ok_or(IssueError::IssueInvalidURL)?
        .collect();
    if path_segments.len() >= 4 && path_segments[0] == "github.com" && path_segments[2] == "issues"
    {
        // Extract organization, repository, and issue id
        let organization = path_segments[1].to_owned();
        let repository = path_segments[2].to_owned();
        let issue_id = path_segments[3]
            .parse::<u32>()
            .map_err(|_| IssueError::IssueInvalidURL)?;

        Ok(IssueInfo {
            organization,
            repository,
            issue_id,
        })
    } else {
        Err(IssueError::IssueInvalidURL)
    }
}
