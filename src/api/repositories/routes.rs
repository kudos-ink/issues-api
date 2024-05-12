use std::convert::Infallible;

use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::auth::with_auth;
use crate::types::PaginationParams;

use super::db::DBRepository;
use super::handlers;
use super::models::QueryParams;
// use crate::pagination::GetPagination;
// use crate::pagination::GetSort;

fn with_db(
    db_pool: impl DBRepository,
) -> impl Filter<Extract = (impl DBRepository,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBRepository) -> BoxedFilter<(impl Reply,)> {
    let repository = warp::path!("repositories"); // TODO: move this to the "organization" endpoint as a subendpoint
    let repository_id = warp::path!("repositories" / i32);
    // let repository_name = warp::path!("repositories" / "name" / String);

    let get_repositories = repository
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and(warp::query::<QueryParams>())
        .and(warp::query::<PaginationParams>())
        .and_then(handlers::all_handler);

    let get_repository = repository_id
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::by_id);

    let create_repository = repository
        .and(with_auth())
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_handler);

    let update_repository = repository_id
        .and(with_auth())
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_access.clone()))
        .and_then(handlers::update_handler);

    let delete_repository = repository_id
        .and(with_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_handler);

    get_repositories
        .or(get_repository)
        .or(create_repository)
        .or(delete_repository)
        .or(update_repository)
        .boxed()
}
