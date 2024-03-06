use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, Reply},
};

use super::{
    db::DBOrganization,
    errors::OrganizationError,
    models::{OrganizationRequest, OrganizationResponse},
};

pub async fn create_organization_handler(
    body: OrganizationRequest,
    db_access: impl DBOrganization,
) -> Result<impl Reply, Rejection> {
    match db_access.get_organization_by_name(&body.name).await? {
        Some(u) => Err(warp::reject::custom(OrganizationError::OrganizationExists(
            u.id,
        )))?,
        None => Ok(json(&OrganizationResponse::of(
            db_access.create_organization(body).await?,
        ))),
    }
}

pub async fn get_organization_handler(
    id: i32,
    db_access: impl DBOrganization,
) -> Result<impl Reply, Rejection> {
    match db_access.get_organization(id).await? {
        None => Err(warp::reject::custom(
            OrganizationError::OrganizationNotFound(id),
        ))?,
        Some(organization) => Ok(json(&OrganizationResponse::of(organization))),
    }
}

pub async fn get_organizations_handler(
    db_access: impl DBOrganization,
) -> Result<impl Reply, Rejection> {
    let organizations = db_access.get_organizations().await?;
    Ok(json::<Vec<_>>(
        &organizations
            .into_iter()
            .map(OrganizationResponse::of)
            .collect(),
    ))
}

pub async fn delete_organization_handler(
    id: i32,
    db_access: impl DBOrganization,
) -> Result<impl Reply, Rejection> {
    match db_access.get_organization(id).await? {
        Some(_) => {
            let _ = &db_access.delete_organization(id).await?;
            Ok(StatusCode::OK)
        }
        None => Err(warp::reject::custom(
            OrganizationError::OrganizationNotFound(id),
        ))?,
    }
}
