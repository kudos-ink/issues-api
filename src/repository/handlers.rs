use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, Reply},
};

use crate::organization::{db::DBOrganization, errors::OrganizationError};

use super::{
    db::DBRepository,
    errors::RepositoryError,
    models::{RepositoryCreateRequest, RepositoryResponse},
};

pub async fn create_repository_handler(
    body: RepositoryCreateRequest,
    db_access: impl DBRepository + DBOrganization,
) -> Result<impl Reply, Rejection> {
    match db_access.get_organization(body.organization_id).await? {
        Some(_) => match db_access.get_repository_by_name(&body.name).await? {
            Some(u) => Err(warp::reject::custom(RepositoryError::RepositoryExists(
                u.id,
            )))?,
            None => Ok(json(&RepositoryResponse::of(
                db_access.create_repository(body).await?,
            ))),
        },
        None => Err(warp::reject::custom(
            OrganizationError::OrganizationNotFound(body.organization_id),
        ))?,
    }
}

pub async fn get_repository_handler(
    id: i32,
    db_access: impl DBRepository,
) -> Result<impl Reply, Rejection> {
    match db_access.get_repository(id).await? {
        None => Err(warp::reject::custom(RepositoryError::RepositoryNotFound(
            id,
        )))?,
        Some(repository) => Ok(json(&RepositoryResponse::of(repository))),
    }
}

pub async fn get_repositories_handler(
    db_access: impl DBRepository,
) -> Result<impl Reply, Rejection> {
    let repositories = db_access.get_repositories().await?;
    Ok(json::<Vec<_>>(
        &repositories
            .into_iter()
            .map(RepositoryResponse::of)
            .collect(),
    ))
}

pub async fn delete_repository_handler(
    id: i32,
    db_access: impl DBRepository,
) -> Result<impl Reply, Rejection> {
    match db_access.get_repository(id).await? {
        Some(_) => {
            let _ = &db_access.delete_repository(id).await?;
            Ok(StatusCode::OK)
        }
        None => Err(warp::reject::custom(RepositoryError::RepositoryNotFound(
            id,
        )))?,
    }
}
