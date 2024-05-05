use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, with_status, Reply},
};

use crate::types::PaginationParams;

use super::{
    db::DBRepository,
    errors::RepositoryError,
    models::{NewRepository, QueryParams, UpdateRepository},
};

pub async fn by_id(id: i32, db_access: impl DBRepository) -> Result<impl Reply, Rejection> {
    match db_access.by_id(id)? {
        None => Err(warp::reject::custom(RepositoryError::NotFound(id)))?,
        Some(repository) => Ok(json(&repository)),
    }
}

pub async fn all_handler(
    db_access: impl DBRepository,
    params: QueryParams,
    pagination: PaginationParams,
) -> Result<impl Reply, Rejection> {
    let repositories = db_access.all(params, pagination)?;
    Ok(json::<Vec<_>>(&repositories))
}

pub async fn create_handler(
    repo: NewRepository,
    db_access: impl DBRepository,
) -> Result<impl Reply, Rejection> {
    match db_access.by_slug(&repo.slug)? {
        Some(r) => Err(warp::reject::custom(RepositoryError::AlreadyExists(r.id))),
        None => Ok(with_status(
            json(&db_access.create(&repo)?),
            StatusCode::CREATED,
        )),
    }
}
pub async fn update_handler(
    id: i32,
    repo: UpdateRepository,
    db_access: impl DBRepository,
) -> Result<impl Reply, Rejection> {
    match db_access.by_id(id)? {
        Some(p) => Ok(with_status(
            json(&db_access.update(p.id, &repo)?),
            StatusCode::OK,
        )),
        None => Err(warp::reject::custom(RepositoryError::NotFound(id))),
    }
}
pub async fn delete_handler(
    id: i32,
    db_access: impl DBRepository,
) -> Result<impl Reply, Rejection> {
    match db_access.by_id(id)? {
        Some(p) => Ok(with_status(json(&db_access.delete(p.id)?), StatusCode::OK)),
        None => Err(warp::reject::custom(RepositoryError::NotFound(id))),
    }
}
