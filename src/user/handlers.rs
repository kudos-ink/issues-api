use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, Reply},
};

use crate::http::{GetPagination, GetSort};

use super::{
    db::DBUser,
    errors::UserError,
    models::{GetUserQuery, NewUser, UserResponse, UserSort, UsersRelations},
};

pub async fn create_user_handler(
    body: NewUser,
    db_access: impl DBUser,
) -> Result<impl Reply, Rejection> {
    match db_access
        .get_user_by_username(&body.username, UsersRelations::default())
        .await?
    {
        Some(u) => Err(warp::reject::custom(UserError::UserExists(u.id)))?,
        None => Ok(json(&UserResponse::of(db_access.create_user(body).await?))),
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
        None => Err(warp::reject::custom(UserError::UserNotFound(id)))?,
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
        None => Err(warp::reject::custom(UserError::UserNotFoundByName(name)))?,
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
    // TODO: validate filters (sort)
    let pagination = filters.validate()?;

    let valid_fields = vec!["id", "username"]; //TODO improve with enum
    let sort = sort.validate(valid_fields)?;

    let user_sort = if sort.sort_by.is_some() {
        UserSort::new(&sort.sort_by.unwrap(), sort.descending.unwrap())
    } else {
        UserSort::default()
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
            Ok(StatusCode::OK)
        }
        None => Err(warp::reject::custom(UserError::UserNotFound(id)))?,
    }
}
