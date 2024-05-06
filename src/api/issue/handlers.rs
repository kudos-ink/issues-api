use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, Reply},
};

use crate::{organization::db::DBOrganization, repository::db::DBRepository};

use super::{
    db::DBIssue,
    errors::IssueError,
    models::{IssueCreateRequest, IssueResponse},
    utils::parse_github_issue_url,
};

pub async fn create_issue_handler(
    body: IssueCreateRequest,
    db_access: impl DBIssue + DBOrganization + DBRepository,
) -> Result<impl Reply, Rejection> {
    match db_access.get_issue_by_url(&body.url).await? {
        Some(u) => Err(warp::reject::custom(IssueError::IssueExists(u.id)))?,
        None => {
            _ = parse_github_issue_url(&body.url)?;
            // TODO: get or create both
            // db_access.create_organization(info.organization);
            // db_access.create_repository(info.repository);
            // Ok(json(&IssueResponse::of(
            //     db_access.create_issue(body).await?,
            // )))
            Ok(StatusCode::OK) //TODO: added to avoid errors in IDE
        }
    }
}

pub async fn get_issue_handler(id: i32, db_access: impl DBIssue) -> Result<impl Reply, Rejection> {
    match db_access.get_issue(id).await? {
        None => Err(warp::reject::custom(IssueError::IssueNotFound(id)))?,
        Some(issue) => Ok(json(&IssueResponse::of(issue))),
    }
}

pub async fn get_issues_handler(db_access: impl DBIssue) -> Result<impl Reply, Rejection> {
    let issues = db_access.get_issues().await?;
    Ok(json::<Vec<_>>(
        &issues.into_iter().map(IssueResponse::of).collect(),
    ))
}

pub async fn delete_issue_handler(
    id: i32,
    db_access: impl DBIssue,
) -> Result<impl Reply, Rejection> {
    match db_access.get_issue(id).await? {
        Some(_) => {
            let _ = &db_access.delete_issue(id).await?;
            Ok(StatusCode::OK)
        }
        None => Err(warp::reject::custom(IssueError::IssueNotFound(id)))?,
    }
}
