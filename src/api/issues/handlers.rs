use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, with_status, Reply},
};

use crate::{api::repositories::db::DBRepository, types::PaginationParams};

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
    let issues = db_access.all(params, pagination)?;
    Ok(json::<Vec<_>>(&issues))
}

pub async fn create_handler(
    issue: NewIssue,
    db_access: impl DBIssue + DBRepository,
) -> Result<impl Reply, Rejection> {
    match DBRepository::by_id(&db_access, issue.repository_id)? {
        Some(_) => match db_access.by_number(issue.repository_id, issue.number)? {
            Some(r) => Err(warp::reject::custom(IssueError::AlreadyExists(r.id))),
            None => Ok(with_status(
                json(&DBIssue::create(&db_access, &issue)?),
                StatusCode::CREATED,
            )),
        },
        None => Err(warp::reject::custom(IssueError::RepositoryNotFound(
            issue.repository_id,
        ))),
    }
}
pub async fn update_handler(
    id: i32,
    issue: UpdateIssue,
    db_access: impl DBIssue + DBRepository,
) -> Result<impl Reply, Rejection> {
    match DBIssue::by_id(&db_access, id)? {
        Some(p) => {
            if let Some(repo_id) = issue.repository_id {
                if DBRepository::by_id(&db_access, repo_id)?.is_none() {
                    return Err(warp::reject::custom(IssueError::RepositoryNotFound(
                        repo_id,
                    )));
                }
            }
            Ok(with_status(
                json(&DBIssue::update(&db_access, p.id, &issue)?),
                StatusCode::OK,
            ))
        }
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
