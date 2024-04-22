use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, with_status, Reply},
};

use crate::{
    pagination::{GetPagination, GetSort},
    repository::db::DBRepository,
};

use super::{
    db::DBUser,
    errors::UserError,
    models::{GetUserQuery, NewUser, PatchUser, UserResponse, UserSort, UsersRelations},
};

pub async fn create_user_handler(
    body: NewUser,
    db_access: impl DBUser + DBRepository,
) -> Result<impl Reply, Rejection> {
    match db_access
        .get_user_by_username(&body.username, UsersRelations::default())
        .await?
    {
        Some(u) => Err(warp::reject::custom(UserError::AlreadyExists(u.id)))?,
        None => {
            if let Some(repositories) = body.repositories.clone() {
                for repo_id in repositories {
                    if db_access.get_repository(repo_id).await?.is_none() {
                        return Err(warp::reject::custom(UserError::CannotBeCreated(format!(
                            "repository {repo_id} does not exist"
                        ))));
                    }
                }
            }
            let new_user = db_access.create_user(body).await?;
            Ok(with_status(
                json(&UserResponse::of(new_user)),
                StatusCode::CREATED,
            ))
        }
    }
}
pub async fn patch_user_handler(
    id: i32,
    body: PatchUser,
    db_access: impl DBUser + DBRepository,
) -> Result<impl Reply, Rejection> {
    match db_access.get_user(id, UsersRelations::default()).await? {
        None => Err(warp::reject::custom(UserError::NotFound(id)))?,
        Some(_) => {
            for repo_id in &body.repositories {
                if db_access.get_repository(*repo_id).await?.is_none() {
                    return Err(warp::reject::custom(UserError::CannotBeUpdated(
                        id,
                        format!("repository {repo_id} does not exist"),
                    )));
                }
            }

            db_access.update_user_maintainers(id, body).await?;
            Ok(StatusCode::NO_CONTENT)
        }
    }
}

pub async fn get_user_handler(
    id: i32,
    db_access: impl DBUser,
    query: GetUserQuery,
) -> Result<impl Reply, Rejection> {
    let relations = UsersRelations {
        wishes: query.wishes.unwrap_or_default(),
        tips: query.tips.unwrap_or_default(),
        maintainers: query.maintainers.unwrap_or_default(),
        issues: query.issues.unwrap_or_default(),
    };

    match db_access.get_user(id, relations).await? {
        None => Err(warp::reject::custom(UserError::NotFound(id)))?,
        Some(user) => Ok(json(&UserResponse::of(user))),
    }
}

pub async fn get_user_by_name_handler(
    name: String,
    db_access: impl DBUser,
    query: GetUserQuery,
) -> Result<impl Reply, Rejection> {
    let relations = UsersRelations {
        wishes: query.wishes.unwrap_or_default(),
        tips: query.tips.unwrap_or_default(),
        maintainers: query.maintainers.unwrap_or_default(),
        issues: query.issues.unwrap_or_default(),
    };

    match db_access.get_user_by_username(&name, relations).await? {
        None => Err(warp::reject::custom(UserError::NotFoundByName(name)))?,
        Some(user) => Ok(json(&UserResponse::of(user))),
    }
}

pub async fn get_users_handler(
    db_access: impl DBUser,
    query: GetUserQuery,
    filters: GetPagination,
    sort: GetSort,
) -> Result<impl Reply, Rejection> {
    let relations = UsersRelations {
        wishes: query.wishes.unwrap_or_default(),
        tips: query.tips.unwrap_or_default(),
        maintainers: query.maintainers.unwrap_or_default(),
        issues: query.issues.unwrap_or_default(),
    };
    let pagination = filters.validate()?;
    let sort = sort.validate()?;
    let user_sort = match (sort.sort_by, sort.descending) {
        (Some(sort_by), Some(descending)) => UserSort::new(&sort_by, descending)?,
        _ => UserSort::default(),
    };

    let users = db_access
        .get_users(relations, pagination, user_sort)
        .await?;
    Ok(json::<Vec<_>>(
        &users.into_iter().map(UserResponse::of).collect(),
    ))
}

pub async fn delete_user_handler(id: i32, db_access: impl DBUser) -> Result<impl Reply, Rejection> {
    match db_access.get_user(id, UsersRelations::default()).await? {
        Some(_) => {
            let _ = &db_access.delete_user(id).await?;
            Ok(StatusCode::NO_CONTENT)
        }
        None => Err(warp::reject::custom(UserError::NotFound(id)))?,
    }
}
