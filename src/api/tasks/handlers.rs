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
        users::db::DBUser,
    },
    middlewares::github::model::GitHubUser,
    types::{PaginatedResponse, PaginationParams},
};

use super::{
    db::DBTask,
    errors::TaskError,
    models::{NewTaskVote, QueryParams, TaskVoteDB, UpdateTask},
};

pub async fn by_id(id: i32, db_access: impl DBTask) -> Result<impl Reply, Rejection> {
    info!("getting task '{id}'");
    match db_access.by_id(id)? {
        None => Err(warp::reject::custom(TaskError::NotFound(id)))?,
        Some(repository) => Ok(json(&repository)),
    }
}

pub async fn all_handler(
    db_access: impl DBTask,
    params: QueryParams,
    pagination: PaginationParams,
) -> Result<impl Reply, Rejection> {
    info!("getting all the roles");
    let (roles, total_count) = db_access.all(params, pagination.clone())?;
    let has_next_page = pagination.offset + pagination.limit < total_count;
    let has_previous_page = pagination.offset > 0;

    let response = PaginatedResponse {
        total_count: Some(total_count),
        has_next_page,
        has_previous_page,
        data: roles,
    };

    Ok(json(&response))
}

pub async fn create_handler(
    user: GitHubUser,
    buf: impl Buf,
    db_access: impl DBTask + DBRole,
) -> Result<impl Reply, Rejection> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let task: NewTask = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid task '{e}'",);
        reject::custom(TaskError::InvalidPayload(e))
    })?;

    // Fetch the user roles from the database
    let user_roles = DBRole::user_roles(&db_access, &user.username)?;
    user_has_at_least_one_role(
        user_roles.clone(),
        vec![
            KudosRole::Admin,
            KudosRole::Contributor,
            KudosRole::MaintainerWithProjects(task.project_id.map(|id| vec![id])),
            KudosRole::EcosystemArchitect,
        ],
    )?;

    if task.assignee_team_id.is_some()
        || task.assignee_user_id.is_some()
        || task.is_certified.is_some()
        || task.featured_by_user_id.is_some()
        || task.is_featured.is_some()
    {
        user_has_at_least_one_role(
            user_roles,
            vec![
                KudosRole::Admin,
                KudosRole::MaintainerWithProjects(task.project_id.map(|id| vec![id])),
                KudosRole::EcosystemArchitect,
            ],
        )?;
    }
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
    db_access: impl DBTask + DBRole,
) -> Result<impl Reply, Rejection> {
    // TODO: check if the user has that task?
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let task: UpdateTask = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid task update: '{e}'",);
        reject::custom(TaskError::InvalidPayload(e))
    })?;

    // Fetch the user roles from the database
    let user_roles = DBRole::user_roles(&db_access, &user.username)?;
    user_has_at_least_one_role(
        user_roles.clone(),
        vec![
            KudosRole::Admin,
            KudosRole::MaintainerWithProjects(task.project_id.map(|id| vec![id])),
            KudosRole::EcosystemArchitect,
        ],
    )?;
    
    task.type_.as_deref().map(validate_task_type).transpose()?;
    match DBTask::by_id(&db_access, id)? {
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
    db_access: impl DBTask + DBRole,
) -> Result<impl Reply, Rejection> {
    let user_roles = DBRole::user_roles(&db_access, &user.username)?;
    user_has_at_least_one_role(
        user_roles,
        vec![
            KudosRole::Admin,
            KudosRole::MaintainerWithProjects(Some(vec![id])),
            KudosRole::EcosystemArchitect,
        ],
    )?;

    match DBTask::by_id(&db_access, id)? {
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

    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let task_vote: NewTaskVote = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid task vote: '{e}'",);
        reject::custom(TaskError::InvalidPayload(e))
    })?;

    match DBUser::by_id(&db_access, task_vote.user_id)? {
        Some(_) => match DBTask::by_id(&db_access, task_vote.task_id)? {
            Some(_) => {
                match DBTask::add_vote_to_task(
                    &db_access,
                    &TaskVoteDB {
                        user_id: task_vote.user_id,
                        task_id: task_vote.task_id,
                        vote: 1,
                    },
                ) {
                    Ok(task_vote) => {
                        info!("vote '{}' created", task_vote.id);
                        Ok(with_status(json(&task_vote), StatusCode::CREATED))
                    }
                    Err(error) => {
                        error!("error creating the vote '{:?}': {}", task_vote, error);
                        if error.to_string().contains("unique_vote") {
                            Err(warp::reject::custom(TaskError::UserAlreadyVoted()))
                        } else {
                            Err(warp::reject::custom(TaskError::CannotCreate(
                                "error creating vote".to_owned(),
                            )))
                        }
                    }
                }
            }
            None => Err(warp::reject::custom(TaskError::NotFound(task_vote.task_id))),
        },
        None => Err(warp::reject::custom(TaskError::UserNotFound(
            task_vote.user_id,
        ))),
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

    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let task_vote: NewTaskVote = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid task vote: '{e}'",);
        reject::custom(TaskError::InvalidPayload(e))
    })?;

    match DBUser::by_id(&db_access, task_vote.user_id)? {
        Some(_) => match DBTask::by_id(&db_access, task_vote.task_id)? {
            Some(_) => {
                match DBTask::add_vote_to_task(
                    &db_access,
                    &TaskVoteDB {
                        user_id: task_vote.user_id,
                        task_id: task_vote.task_id,
                        vote: -1,
                    },
                ) {
                    Ok(task_vote) => {
                        info!("vote '{}' created", task_vote.id);
                        Ok(with_status(json(&task_vote), StatusCode::CREATED))
                    }
                    Err(error) => {
                        error!("error creating the vote '{:?}': {}", task_vote, error);
                        if error.to_string().contains("unique_vote") {
                            Err(warp::reject::custom(TaskError::UserAlreadyVoted()))
                        } else {
                            Err(warp::reject::custom(TaskError::CannotCreate(
                                "error creating vote".to_owned(),
                            )))
                        }
                    }
                }
            }
            None => Err(warp::reject::custom(TaskError::NotFound(task_vote.task_id))),
        },
        None => Err(warp::reject::custom(TaskError::UserNotFound(
            task_vote.user_id,
        ))),
    }
}
pub async fn delete_task_vote(
    id: i32,
    _: GitHubUser,
    db_access: impl DBTask,
) -> Result<impl Reply, Rejection> {
    // TODO: check if the user has that vote
    match db_access.delete_task_vote(id) {
        Ok(role) => {
            info!("task vote '{}' deleted", id);
            Ok(with_status(json(&role), StatusCode::NO_CONTENT))
        }
        Err(error) => {
            error!("error deleting the task vote '{id}': {error}");
            Err(warp::reject::custom(TaskError::CannotDelete(
                "error deleting the task vote".to_owned(),
            )))
        }
    }
}
