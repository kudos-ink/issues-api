use std::convert::Infallible;

use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::organization::db::DBOrganization;

use super::db::DBRepository;
use super::handlers;

fn with_db(
    db_pool: impl DBRepository + DBOrganization,
) -> impl Filter<Extract = (impl DBRepository + DBOrganization,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBRepository + DBOrganization) -> BoxedFilter<(impl Reply,)> {
    let repository = warp::path!("repositories"); // TODO: move this to the "organization" endpoint as a subendpoint
    let repository_id = warp::path!("repositories" / i32);

    let get_repositories = repository
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::get_repositories_handler);

    let get_repository = repository_id
        .and(warp::get())
        // .and(warp::path::param())
        .and(with_db(db_access.clone()))
        .and_then(handlers::get_repository_handler);

    let create_repository = repository
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_repository_handler);

    let delete_repository = repository_id
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_repository_handler);

    let route = get_repositories
        .or(get_repository)
        .or(create_repository)
        .or(delete_repository);

    route.boxed()
}
