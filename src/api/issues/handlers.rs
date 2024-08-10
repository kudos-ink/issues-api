use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, with_status, Reply},
};

use crate::{
    api::repositories::db::DBRepository,
    types::{PaginatedResponse, PaginationParams},
};

use super::{
    db::DBIssue,
    errors::IssueError,
    models::{NewIssue, QueryParams, UpdateIssue},
};

pub async fn by_id(id: i32, db_access: impl DBIssue) -> Result<impl Reply, Rejection> {
    match db_access.by_id(id)? {
        None => Err(warp::reject::custom(IssueError::NotFound(id)))?,
        Some(repository) => Ok(json(&repository)),
    }
}

pub async fn all_handler(
    db_access: impl DBIssue,
    params: QueryParams,
    pagination: PaginationParams,
) -> Result<impl Reply, Rejection> {
    let (issues, total_count) = db_access.all(params, pagination.clone())?;
    let has_next_page = pagination.offset + pagination.limit < total_count;
    let has_previous_page = pagination.offset > 0;

    let response = PaginatedResponse {
        total_count: Some(total_count),
        has_next_page,
        has_previous_page,
        data: issues,
    };

    Ok(json(&response))
}

pub async fn create_handler(
    issue: NewIssue,
    db_access: impl DBIssue + DBRepository,
) -> Result<impl Reply, Rejection> {
    match DBRepository::by_id(&db_access, issue.repository_id) {
        Ok(_) => match db_access.by_number(issue.repository_id, issue.number)? {
            Some(r) => Err(warp::reject::custom(IssueError::AlreadyExists(r.id))),
            None => Ok(with_status(
                // check if repository exists
                json(&DBIssue::create(&db_access, &issue)?),
                StatusCode::CREATED,
            )),
        },
        Err(_) => Err(warp::reject::custom(IssueError::InvalidPayload)),
    }
}
pub async fn update_handler(
    id: i32,
    issue: UpdateIssue,
    db_access: impl DBIssue + DBRepository,
) -> Result<impl Reply, Rejection> {
    if let Some(repo_id) = issue.repository_id {
        if DBRepository::by_id(&db_access, repo_id).is_err() {
            return Err(warp::reject::custom(IssueError::InvalidPayload));
        }
    }

    match DBIssue::by_id(&db_access, id)? {
        Some(p) => Ok(with_status(
            json(&DBIssue::update(&db_access, p.id, &issue)?),
            StatusCode::OK,
        )),
        None => Err(warp::reject::custom(IssueError::NotFound(id))),
    }
}
pub async fn delete_handler(id: i32, db_access: impl DBIssue) -> Result<impl Reply, Rejection> {
    match DBIssue::by_id(&db_access, id)? {
        Some(_) => {
            let _ = &db_access.delete(id)?;
            Ok(StatusCode::OK)
        }
        None => Err(warp::reject::custom(IssueError::NotFound(id)))?,
    }
}
