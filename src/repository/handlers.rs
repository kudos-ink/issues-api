use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, with_status, Reply},
};

use crate::{
    organization::{db::DBOrganization, errors::OrganizationError},
    pagination::GetSort,
};

use super::{
    db::DBRepository,
    errors::RepositoryError,
    models::{GetRepositoryQuery, NewRepository, RepositoriesRelations, RepositoryResponse},
};
use crate::pagination::GetPagination;
use crate::repository::models::RepositorySort;

pub async fn create_repository_handler(
    body: NewRepository,
    db_access: impl DBRepository + DBOrganization,
) -> Result<impl Reply, Rejection> {
    match db_access.get_organization(body.organization_id).await? {
        Some(_) => match db_access
            .get_repository_by_name(&body.name, RepositoriesRelations::default())
            .await?
        {
            Some(u) => Err(warp::reject::custom(RepositoryError::AlreadyExists(u.id)))?,
            None => Ok(with_status(
                json(&RepositoryResponse::of(
                    db_access.create_repository(body).await?,
                )),
                StatusCode::CREATED,
            )),
        },
        None => Err(warp::reject::custom(
            OrganizationError::OrganizationNotFound(body.organization_id),
        ))?,
    }
}

pub async fn get_repository_handler(
    id: i32,
    db_access: impl DBRepository,
    query: GetRepositoryQuery,
) -> Result<impl Reply, Rejection> {
    let relations = RepositoriesRelations {
        tips: query.tips.unwrap_or_default(),
        maintainers: query.maintainers.unwrap_or_default(),
        issues: query.issues.unwrap_or_default(),
        languages: query.languages.unwrap_or_default(),
    };
    match db_access.get_repository(id, relations).await? {
        None => Err(warp::reject::custom(RepositoryError::NotFound(id)))?,
        Some(repository) => Ok(json(&RepositoryResponse::of(repository))),
    }
}
pub async fn get_repository_handler_name(
    name: String,
    db_access: impl DBRepository,
    query: GetRepositoryQuery,
) -> Result<impl Reply, Rejection> {
    let relations = RepositoriesRelations {
        tips: query.tips.unwrap_or_default(),
        maintainers: query.maintainers.unwrap_or_default(),
        issues: query.issues.unwrap_or_default(),
        languages: query.languages.unwrap_or_default(),
    };
    match db_access.get_repository_by_name(&name, relations).await? {
        None => Err(warp::reject::custom(RepositoryError::NotFoundByName(name)))?,
        Some(repository) => Ok(json(&RepositoryResponse::of(repository))),
    }
}

pub async fn get_repositories_handler(
    db_access: impl DBRepository,
    query: GetRepositoryQuery,
    filters: GetPagination,
    sort: GetSort,
) -> Result<impl Reply, Rejection> {
    let relations = RepositoriesRelations {
        tips: query.tips.unwrap_or_default(),
        maintainers: query.maintainers.unwrap_or_default(),
        issues: query.issues.unwrap_or_default(),
        languages: query.languages.unwrap_or_default(),
    };
    let pagination = filters.validate()?;
    let sort = sort.validate()?;
    let repository_sort = match (sort.sort_by, sort.descending) {
        (Some(sort_by), Some(descending)) => RepositorySort::new(&sort_by, descending)?,
        _ => RepositorySort::default(),
    };

    let repositories = db_access
        .get_repositories(relations, pagination, repository_sort)
        .await?;
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
    match db_access
        .get_repository(id, RepositoriesRelations::default())
        .await?
    {
        Some(_) => {
            let _ = &db_access.delete_repository(id).await?;
            Ok(StatusCode::NO_CONTENT)
        }
        None => Err(warp::reject::custom(RepositoryError::NotFound(id)))?,
    }
}
