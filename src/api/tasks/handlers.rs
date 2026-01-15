use bytes::Buf;
use log::{error, info, warn};
use warp::{
    http::StatusCode,
    reject,
    reject::Rejection,
    reply::{json, with_status, Reply},
};

use crate::{
    api::{
        roles::{db::DBRole, models::KudosRole, utils::user_has_at_least_one_role},
        tasks::{models::NewTask, utils::validate_task_type},
        users::{db::DBUser, errors::UserError},
    },
    middlewares::github::model::GitHubUser,
    types::{PaginatedResponse, PaginationParams},
};

use super::{
    db::DBTask,
    errors::TaskError,
    models::{VotePayload, QueryParams, TaskVoteDB, UpdateTask},
};

async fn get_user_id_from_auth(
    github_user: Option<GitHubUser>, 
    db_access: &impl DBUser
) -> Result<Option<i32>, Rejection> {
    if let Some(user) = github_user {
        match db_access.by_github_id(user.id)? {
            Some(db_user) => Ok(Some(db_user.id)),
            None => Ok(None),
        }
    } else {
        Ok(None)
    }
}


pub async fn by_id(id: i32, github_user: Option<GitHubUser>, db_access: impl DBTask + DBUser) -> Result<impl Reply, Rejection> {
    info!("getting task '{id}'");
    let current_user_id = get_user_id_from_auth(github_user, &db_access).await?;
    match DBTask::by_id(&db_access, id, current_user_id)? {
        None => Err(warp::reject::custom(TaskError::NotFound(id)))?,
        Some(task) => Ok(json(&task)),
    }
}

pub async fn all_handler(
    github_user: Option<GitHubUser>,
    db_access: impl DBTask + DBUser,
    params: QueryParams,
    pagination: PaginationParams,
) -> Result<impl Reply, Rejection> {
    info!("getting all the roles");
    let current_user_id = get_user_id_from_auth(github_user, &db_access).await?;
    let (tasks, total_count) = DBTask::all(&db_access, params, pagination.clone(), current_user_id)?;
    let has_next_page = pagination.offset + pagination.limit < total_count;
    let has_previous_page = pagination.offset > 0;

    let response = PaginatedResponse {
        total_count: Some(total_count),
        has_next_page,
        has_previous_page,
        data: tasks,
    };

    Ok(json(&response))
}

pub async fn create_handler(
    user: GitHubUser,
    buf: impl Buf,
    db_access: impl DBTask + DBRole + DBUser,
) -> Result<impl Reply, Rejection> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let mut task: NewTask = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid task '{e}'",);
        reject::custom(TaskError::InvalidPayload(e))
    })?;

    // Fetch the user roles from the database
    let user_roles = DBRole::user_roles(&db_access, &user.username)?;

    if task.assignee_team_id.is_some()
        || task.assignee_user_id.is_some()
        || task.is_certified.is_some()
        || task.featured_by_user_id.is_some()
        || task.is_featured.is_some()
        || task.repository_id.is_some()
        || task.project_id.is_some()
        || task.approved_by.is_some()
        || task.featured_by_user_id.is_some()
    {
        user_has_at_least_one_role(
            user_roles,
            vec![
                KudosRole::Admin,
                KudosRole::MaintainerWithProjects(task.project_id.map(|id| vec![id])),
                KudosRole::EcosystemArchitect,
            ],
        )?;
    } else {
        user_has_at_least_one_role(
            user_roles.clone(),
            vec![
                KudosRole::Admin,
                KudosRole::Contributor,
                KudosRole::MaintainerWithProjects(task.project_id.map(|id| vec![id])),
                KudosRole::EcosystemArchitect,
            ],
        )?;
    }
    let user = DBUser::by_username(&db_access, &user.username)?
        .ok_or_else(|| UserError::NotFoundByName(user.username.to_owned()))?;
    task.created_by_user_id = Some(user.id);

    validate_task_type(&task.type_)?;
    info!("creating task '{}'", task.title);
    // TODO: validate
    match DBTask::create(&db_access, &task) {
        Ok(task) => {
            info!("task id '{}' created", task.id);
            Ok(with_status(json(&task), StatusCode::CREATED))
        }
        Err(err) => {
            error!("error creating the task '{:?}': {}", task, err);
            if err.to_string().contains("tasks_project_id_fkey") {
                Err(warp::reject::custom(TaskError::ProjectNotFound(
                    task.project_id.unwrap(),
                )))
            } else {
                Err(warp::reject::custom(TaskError::CannotCreate(
                    "error creating the task".to_owned(), // TODO: improve errors
                )))
            }
        }
    }
}
pub async fn update_handler(
    id: i32,
    user: GitHubUser,
    buf: impl Buf,
    db_access: impl DBTask + DBRole + DBUser,
) -> Result<impl Reply, Rejection> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let task: UpdateTask = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid task update: '{e}'",);
        reject::custom(TaskError::InvalidPayload(e))
    })?;

    let user_roles = DBRole::user_roles(&db_access, &user.username)?;

    user_has_at_least_one_role(
        user_roles.clone(),
        vec![
            KudosRole::Admin,
            KudosRole::MaintainerWithProjects(task.project_id.map(|id| vec![id])),
            KudosRole::EcosystemArchitect,
        ],
    )?;

    let db_user = db_access.by_github_id(user.id)?
        .ok_or_else(|| reject::custom(UserError::GithubNotFound(user.id)))?;

    task.type_.as_deref().map(validate_task_type).transpose()?;
    match DBTask::by_id(&db_access, id, Some(db_user.id))? {
        Some(p) => match DBTask::update(&db_access, p.id, &task) {
            Ok(task) => {
                info!("task '{}' updated", task.id);
                Ok(with_status(json(&task), StatusCode::OK))
            }
            Err(error) => {
                error!("error updating the task '{:?}': {}", task, error);

                Err(warp::reject::custom(TaskError::CannotUpdate(
                    "error updating the task".to_owned(),
                )))
            }
        },
        None => Err(warp::reject::custom(TaskError::NotFound(id))),
    }
}
pub async fn delete_handler(
    id: i32,
    user: GitHubUser,
    db_access: impl DBTask + DBRole + DBUser,
) -> Result<impl Reply, Rejection> {
    let user_roles = DBRole::user_roles(&db_access, &user.username)?;
    user_has_at_least_one_role(
        user_roles,
        vec![
            KudosRole::Admin,
            KudosRole::EcosystemArchitect,
        ],
    )?;

    let db_user = db_access.by_github_id(user.id)?
        .ok_or_else(|| reject::custom(UserError::GithubNotFound(user.id)))?;

    match DBTask::by_id(&db_access, id, Some(db_user.id))? {
        Some(_) => {
            let _ = &DBTask::delete(&db_access, id)?;
            Ok(StatusCode::NO_CONTENT)
        }
        None => Err(warp::reject::custom(TaskError::NotFound(id)))?,
    }
}

pub async fn add_upvote_to_task(
    user: GitHubUser,
    buf: impl Buf,
    db_access: impl DBTask + DBUser + DBRole,
) -> Result<impl Reply, Rejection> {
    let user_roles = DBRole::user_roles(&db_access, &user.username)?;
    user_has_at_least_one_role(
        user_roles,
        vec![
            KudosRole::Admin,
            KudosRole::Contributor,
            KudosRole::MaintainerWithProjects(None),
            KudosRole::EcosystemArchitect,
        ],
    )?;

    let db_user = db_access.by_github_id(user.id)?
        .ok_or_else(|| reject::custom(UserError::GithubNotFound(user.id)))?;

    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let task_vote: VotePayload = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid task vote: '{e}'",);
        reject::custom(TaskError::InvalidPayload(e))
    })?;

    match DBTask::by_id(&db_access, task_vote.task_id, Some(db_user.id))? {
        Some(_) => {
            // 4. Create the vote using the SECURE user ID from the backend.
            match DBTask::add_vote_to_task(
                &db_access,
                &TaskVoteDB {
                    user_id: db_user.id, // <-- Use the secure ID
                    task_id: task_vote.task_id,
                    vote: 1, // 1 for upvote
                },
            ) {
                Ok(task_vote) => {
                    info!("vote '{}' created/updated", task_vote.id);
                    Ok(with_status(json(&task_vote), StatusCode::OK)) // Use OK for upsert
                }
                Err(error) => {
                    error!("error creating the vote: {}", error);
                    Err(reject::custom(TaskError::CannotCreate(
                        "error creating vote".to_owned(),
                    )))
                }
            }
        }
        None => Err(reject::custom(TaskError::NotFound(task_vote.task_id))),
    }
}
pub async fn add_downvote_to_task(
    user: GitHubUser,
    buf: impl Buf,
    db_access: impl DBTask + DBUser + DBRole,
) -> Result<impl Reply, Rejection> {
    let user_roles = DBRole::user_roles(&db_access, &user.username)?;
    user_has_at_least_one_role(
        user_roles,
        vec![
            KudosRole::Admin,
            KudosRole::Contributor,
            KudosRole::MaintainerWithProjects(None),
            KudosRole::EcosystemArchitect,
        ],
    )?;

    let db_user = db_access.by_github_id(user.id)?
        .ok_or_else(|| reject::custom(UserError::GithubNotFound(user.id)))?;

    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let task_vote: VotePayload = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid task vote: '{e}'",);
        reject::custom(TaskError::InvalidPayload(e))
    })?;

    match DBTask::by_id(&db_access, task_vote.task_id, Some(db_user.id))? {
        Some(_) => {
            // 4. Create the vote using the SECURE user ID from the backend.
            match DBTask::add_vote_to_task(
                &db_access,
                &TaskVoteDB {
                    user_id: db_user.id, // <-- Use the secure ID
                    task_id: task_vote.task_id,
                    vote: -1,
                },
            ) {
                Ok(task_vote) => {
                    info!("vote '{}' created/updated", task_vote.id);
                    Ok(with_status(json(&task_vote), StatusCode::OK)) // Use OK for upsert
                }
                Err(error) => {
                    error!("error creating the vote: {}", error);
                    Err(reject::custom(TaskError::CannotCreate(
                        "error creating vote".to_owned(),
                    )))
                }
            }
        }
        None => Err(reject::custom(TaskError::NotFound(task_vote.task_id))),
    }
}
// pub async fn delete_task_vote(
//     id: i32,
//     _: GitHubUser,
//     db_access: impl DBTask,
// ) -> Result<impl Reply, Rejection> {
//     // TODO: check if the user has that vote
//     match db_access.delete_task_vote(id) {
//         Ok(role) => {
//             info!("task vote '{}' deleted", id);
//             Ok(with_status(json(&role), StatusCode::NO_CONTENT))
//         }
//         Err(error) => {
//             error!("error deleting the task vote '{id}': {error}");
//             Err(warp::reject::custom(TaskError::CannotDelete(
//                 "error deleting the task vote".to_owned(),
//             )))
//         }
//     }
// }

pub async fn delete_vote_handler(
    user: GitHubUser,
    buf: impl Buf,
    db_access: impl DBTask + DBUser,
) -> Result<impl Reply, Rejection> {

    let db_user = db_access.by_github_id(user.id)?
        .ok_or_else(|| reject::custom(UserError::GithubNotFound(user.id)))?;


    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let vote_payload: VotePayload = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid task vote delete payload: '{e}'");
        reject::custom(TaskError::InvalidPayload(e))
    })?;


    match db_access.delete_vote_by_user_and_task(db_user.id, vote_payload.task_id) {
        Ok(0) => {
            warn!("No vote found to delete for user #{} on task #{}", db_user.id, vote_payload.task_id);
            Ok(StatusCode::NO_CONTENT)
        }
        Ok(_) => {
            info!("Vote deleted for user #{} on task #{}", db_user.id, vote_payload.task_id);
            Ok(StatusCode::NO_CONTENT)
        }
        Err(e) => {
            error!("Failed to delete vote: {}", e);
            Err(reject::custom(TaskError::CannotDelete("Failed to delete vote".into())))
        }
    }
}

