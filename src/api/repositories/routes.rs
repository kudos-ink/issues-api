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
    let repository = warp::path!("repositories");
    let repository_id = warp::path!("repositories" / i32);

    let all_route = repository
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and(warp::query::<QueryParams>())
        .and(warp::query::<PaginationParams>())
        .and_then(handlers::all_handler);

    let by_id_route = repository_id
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::by_id);

    let create_route = repository
        .and(with_auth())
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_handler);

    let update_route = repository_id
        .and(with_auth())
        .and(warp::patch())
        .and(warp::body::json())
        .and(with_db(db_access.clone()))
        .and_then(handlers::update_handler);

    let delete_route = repository_id
        .and(with_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_handler);

    let languages_route = warp::path!("languages")
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::get_languages_handler);

    all_route
        .or(by_id_route)
        .or(create_route)
        .or(update_route)
        .or(delete_route)
        .or(languages_route)
        .boxed()
}
