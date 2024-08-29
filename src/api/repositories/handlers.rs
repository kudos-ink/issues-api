use warp::{
    http::StatusCode,
    reject::Rejection,
    reject,
    reply::{json, with_status, Reply},
};
use log::{error, info, warn};
use bytes::Buf;

use crate::{api::projects::db::DBProject, types::{PaginatedResponse, PaginationParams}};

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
    let repositories = db_access.all(params, pagination.clone())?;
    let total_count = repositories.len() as i64;
    let has_next_page = pagination.offset + pagination.limit < total_count;
    let has_previous_page = pagination.offset > 0;

    let response = PaginatedResponse {
        total_count: Some(total_count),
        has_next_page,
        has_previous_page,
        data: repositories,
    };

    Ok(json(&response))
}

pub async fn create_handler(
    buf: impl Buf,
    db_access: impl DBRepository + DBProject,
) -> Result<impl Reply, Rejection> {

    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let repository: NewRepository = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid repository '{e}'",);
        reject::custom(RepositoryError::InvalidPayload(e))
    })?;
    match DBRepository::by_slug(&db_access, &repository.slug)? {
        Some(r) => Err(warp::reject::custom(RepositoryError::AlreadyExists(r.id))),
        None =>  match DBProject::by_id(&db_access, repository.project_id) {
            Ok(project) => match project {
                Some(_) => match DBRepository::create(&db_access, &repository){
                    Ok(_) => {
                        info!("repository slug '{}' created", repository.slug);
                        Ok(with_status(json(&repository), StatusCode::CREATED))},
                    Err(err) =>  {
                        error!("error creating the repository '{:?}': {}", repository, err);
                        Err(warp::reject::custom(RepositoryError::CannotCreate(
                            "error creating the repository".to_owned(),
                        )))
                    },
                },
                None => {
                    warn!("project id '{}' does not exist", repository.project_id);
                    Err(warp::reject::custom(RepositoryError::ProjectNotFound(repository.project_id)))
                },
            },
            Err(_) => Err(warp::reject::custom(RepositoryError::CannotCreate(
                "cannot check if the repository exists".to_owned(),
            ))),
        }
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

pub async fn get_languages_handler(db_access: impl DBRepository) -> Result<impl Reply, Rejection> {
    let languages = db_access.aggregate_languages()?;
    Ok(json(&languages))
}
