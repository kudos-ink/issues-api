use crate::types::PaginationParams;

use super::{
    db::DBProject,
    errors::ProjectError,
    models::{NewProject, QueryParams, UpdateProject},
};
use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, with_status, Reply},
};

pub async fn all_handler(
    db_access: impl DBProject,
    params: QueryParams,
    pagination: PaginationParams,
) -> Result<impl Reply, Rejection> {
    let projects = db_access.all(params, pagination)?;
    Ok(json::<Vec<_>>(&projects))
}

pub async fn create_handler(
    form: NewProject,
    db_access: impl DBProject,
) -> Result<impl Reply, Rejection> {
    match db_access.by_slug(&form.slug)? {
        Some(p) => Err(warp::reject::custom(ProjectError::AlreadyExists(p.id))),
        None => Ok(with_status(
            json(&db_access.create(&form)?),
            StatusCode::CREATED,
        )),
    }
}

pub async fn update_handler(
    id: i32,
    form: UpdateProject,
    db_access: impl DBProject,
) -> Result<impl Reply, Rejection> {
    match db_access.by_id(id)? {
        Some(p) => Ok(with_status(
            json(&db_access.update(p.id, &form)?),
            StatusCode::OK,
        )),
        None => Err(warp::reject::custom(ProjectError::NotFound(id))),
    }
}

pub async fn delete_handler(id: i32, db_access: impl DBProject) -> Result<impl Reply, Rejection> {
    match db_access.by_id(id)? {
        Some(p) => Ok(with_status(json(&db_access.delete(p.id)?), StatusCode::OK)),
        None => Err(warp::reject::custom(ProjectError::NotFound(id))),
    }
}
