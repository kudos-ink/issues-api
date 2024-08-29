use crate::types::{PaginatedResponse, PaginationParams};

use super::{
    db::DBProject,
    errors::ProjectError,
    models::{NewProject, QueryParams, UpdateForm},
};
use warp::{
    http::StatusCode,
    reject::Rejection,
    reject,
    reply::{json, with_status, Reply},
};
use log::{error, info, warn};
use bytes::Buf;

pub async fn all_handler(
    db_access: impl DBProject,
    params: QueryParams,
    pagination: PaginationParams,
) -> Result<impl Reply, Rejection> {
    let (projects, total_count) = db_access.all(params, pagination.clone())?;
    let has_next_page = pagination.offset + pagination.limit < total_count;
    let has_previous_page = pagination.offset > 0;

    let response = PaginatedResponse {
        total_count: Some(total_count),
        has_next_page,
        has_previous_page,
        data: projects,
    };

    Ok(json(&response))
}

pub async fn create_handler(
    buf: impl Buf,
    db_access: impl DBProject,
) -> Result<impl Reply, Rejection> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let project: NewProject = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid project '{e}'",);
        reject::custom(ProjectError::InvalidPayload(e))
    })?;
    match db_access.by_slug(&project.slug)? {
        Some(p) => Err(warp::reject::custom(ProjectError::AlreadyExists(p.id))),
        None => match db_access.create(&project) {
            Ok(project) => {
            info!("project slug '{}' created", project.slug);
            Ok(with_status(
                json(&project),
                StatusCode::CREATED,
            ))
        },
            Err(error) => {
                error!("error creating the project '{:?}': {}", project, error);
                Err(warp::reject::custom(ProjectError::CannotCreate(
                    "error creating the project".to_string(),
                )))
            },
            },
        }
}

pub async fn update_handler(
    id: i32,
    form: UpdateForm,
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
