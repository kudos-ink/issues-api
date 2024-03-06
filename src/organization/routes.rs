use std::convert::Infallible;

use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use super::db::DBOrganization;
use super::handlers;

fn with_db(
    db_pool: impl DBOrganization,
) -> impl Filter<Extract = (impl DBOrganization,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBOrganization) -> BoxedFilter<(impl Reply,)> {
    let organization = warp::path!("organizations");
    let organization_id = warp::path!("organizations" / i32);

    let get_organizations = organization
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::get_organizations_handler);

    let get_organization = organization_id
        .and(warp::get())
        // .and(warp::path::param())
        .and(with_db(db_access.clone()))
        .and_then(handlers::get_organization_handler);

    let create_organization = organization
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_organization_handler);

    let delete_organization = organization_id
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_organization_handler);

    let route = get_organizations
        .or(get_organization)
        .or(create_organization)
        .or(delete_organization);

    route.boxed()
}
