use bytes::Buf;
use log::{error, info, warn};
use warp::{
    http::StatusCode,
    reject,
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
    info!("getting issues '{id}'");
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
    info!("getting all the issues");
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
    buf: impl Buf,
    db_access: impl DBIssue + DBRepository,
) -> Result<impl Reply, Rejection> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let issue: NewIssue = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid issue '{e}'",);
        reject::custom(IssueError::InvalidPayload(e))
    })?;

    info!("creating issue id '{}'", issue.id);
    match DBRepository::by_id(&db_access, issue.repository_id) {
        Ok(repo) => match repo {
            Some(_) => match db_access.by_number(issue.repository_id, issue.number)? {
                Some(r) => {
                    warn!("issue id '{}' exists", issue.id);
                    Err(warp::reject::custom(IssueError::AlreadyExists(r.id)))
                }
                None => match DBIssue::create(&db_access, &issue) {
                    Ok(issue) => {
                        info!("issue id '{}' created", issue.id);
                        Ok(with_status(json(&issue), StatusCode::CREATED))
                    }
                    Err(err) => {
                        error!("error creating the issue '{:?}': {}", issue, err);
                        Err(warp::reject::custom(IssueError::CannotCreate(
                            "error creating the issue".to_owned(),
                        )))
                    }
                },
            },
            None => {
                warn!("repository '{}' invalid", issue.repository_id);
                Err(warp::reject::custom(IssueError::RepositoryNotFound(
                    issue.repository_id
                )))
            }
        },
        Err(_) => Err(warp::reject::custom(IssueError::CannotCreate(
            "cannot check if the repository is valid".to_owned(),
        ))),
    }
}
pub async fn update_handler(
    id: i32,
    issue: UpdateIssue,
    db_access: impl DBIssue + DBRepository,
) -> Result<impl Reply, Rejection> {
    if let Some(repo_id) = issue.repository_id {
        if DBRepository::by_id(&db_access, repo_id).is_err() {
            return Err(warp::reject::custom(IssueError::InvalidPayload(
                "invalid".to_owned(),
            )));
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
