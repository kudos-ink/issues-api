use std::collections::HashMap;

use bytes::Buf;
use log::{error, info, warn};
use warp::{
    http::StatusCode,
    reject,
    reject::Rejection,
    reply::{json, with_status, Reply},
};

use crate::{
    api::{issues::models::LeaderboardEntry, repositories::db::DBRepository, users::db::DBUser},
    types::{PaginatedResponse, PaginationParams},
};

use super::{
    db::DBIssue,
    errors::IssueError,
    models::{IssueAssignee, LeaderboardQueryParams, NewIssue, QueryParams, UpdateIssue},
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
pub async fn leaderboard(
    db_access: impl DBIssue,
    params: LeaderboardQueryParams,
) -> Result<impl Reply, Rejection> {
    info!("getting the leaderboard");
    let (issues, _) = db_access.all(
        QueryParams {
            slug: None,
            certified: params.certified,
            purposes: params.purposes,
            stack_levels: params.stack_levels,
            technologies: params.technologies,
            labels: params.labels,
            language_slug: params.language_slug,
            repository_id: params.repository_id,
            assignee_id: None,
            open: Some(false),
            has_assignee: Some(true),
            issue_closed_at_min: params.start_date,
            issue_closed_at_max: params.close_date,
        },
        PaginationParams {
            limit: i64::max_value(),
            offset: 0,
        },
    )?;
    let mut scores = HashMap::new();

    issues.into_iter().for_each(|issue| {
        scores
            .entry(issue.assignee_username)
            .and_modify(|count| *count += 1)
            .or_insert(1); // Start the count at 1, not 0.
    });

    let leaderboard: Vec<LeaderboardEntry> = scores
        .into_iter()
        .map(|(username, score)| LeaderboardEntry {
            username: username.unwrap_or_default(),
            score,
        })
        .collect();

    Ok(json(&leaderboard))
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

    info!("creating issue number '{}'", issue.number);
    match DBRepository::by_id(&db_access, issue.repository_id) {
        Ok(repo) => match repo {
            Some(_) => match db_access.by_number(issue.repository_id, issue.number)? {
                Some(r) => {
                    warn!("issue number '{}' exists", issue.number);
                    Err(warp::reject::custom(IssueError::AlreadyExists(r.number)))
                }
                None => match DBIssue::create(&db_access, &issue) {
                    Ok(issue) => {
                        info!("issue number '{}' created", issue.number);
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
                    issue.repository_id,
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
    buf: impl Buf,
    db_access: impl DBIssue,
) -> Result<impl Reply, Rejection> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let issue: UpdateIssue = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid issue update: '{e}'",);
        reject::custom(IssueError::InvalidPayload(e))
    })?;
    if !issue.has_any_field() {
        let e = "all the fields are empty";
        warn!("invalid issue update: '{e}'",);
        return Err(reject::custom(IssueError::InvalidPayload(e.to_string())));
    }
    match DBIssue::by_id(&db_access, id)? {
        Some(p) => match db_access.update(p.id, &issue) {
            Ok(issue) => {
                info!("issue '{}' updated", issue.id);
                Ok(with_status(json(&issue), StatusCode::OK))
            }
            Err(error) => {
                error!("error updating the issue '{:?}': {}", issue, error);
                if error.to_string().contains("issue_closed_at_check") {
                    Err(warp::reject::custom(IssueError::InvalidPayload(
                        "issue_closed_at is lower than issue_created_at".to_owned(),
                    )))
                } else {
                    Err(warp::reject::custom(IssueError::CannotUpdate(
                        "error updating the issue".to_owned(),
                    )))
                }
            }
        },
        None => Err(warp::reject::custom(IssueError::NotFound(id))),
    }
}
pub async fn delete_handler(id: i32, db_access: impl DBIssue) -> Result<impl Reply, Rejection> {
    match DBIssue::by_id(&db_access, id)? {
        Some(_) => {
            let _ = &db_access.delete(id)?;
            Ok(StatusCode::NO_CONTENT)
        }
        None => Err(warp::reject::custom(IssueError::NotFound(id)))?,
    }
}

pub async fn update_asignee_handler(
    id: i32,
    buf: impl Buf,
    db_access: impl DBIssue + DBUser,
) -> Result<impl Reply, Rejection> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let assignee: IssueAssignee = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid issue assignee: '{e}'",);
        reject::custom(IssueError::InvalidPayload(e))
    })?;

    match DBUser::by_username(&db_access, &assignee.username)? {
        Some(u) => {
            let update_issue = UpdateIssue {
                assignee_id: Some(u.id),
                ..Default::default()
            };
            match DBIssue::update(&db_access, id, &update_issue) {
                Ok(issue) => {
                    info!("issue '{}' assignee '{}' updated", issue.id, u.id);
                    Ok(with_status(json(&issue), StatusCode::OK))
                }
                Err(error) => {
                    error!("error updating the issue '{id}': {error}");
                    Err(warp::reject::custom(IssueError::CannotUpdate(
                        "error updating the issue assignee".to_owned(),
                    )))
                }
            }
        }
        None => Err(warp::reject::custom(IssueError::InvalidPayload(
            "username not found".to_string(),
        ))),
    }
}
pub async fn delete_asignee_handler(
    id: i32,
    db_access: impl DBIssue + DBUser,
) -> Result<impl Reply, Rejection> {
    match db_access.delete_issue_assignee(id) {
        Ok(issue) => {
            info!("issue '{}' assignee deleted", id);
            Ok(with_status(json(&issue), StatusCode::NO_CONTENT))
        }
        Err(error) => {
            error!("error deleteing the issue '{id}' assignee: {error}");
            Err(warp::reject::custom(IssueError::CannotUpdate(
                "error deleting the issue assignee".to_owned(),
            )))
        }
    }
}
