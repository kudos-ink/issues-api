use std::convert::Infallible;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::api::roles::db::DBRole;
use crate::middlewares::github::auth::with_github_auth;
use crate::types::PaginationParams;

use super::db::DBProject;
use super::handlers;
use super::models::QueryParams;

fn with_db(
    db_pool: impl DBProject + DBRole,
) -> impl Filter<Extract = (impl DBProject + DBRole,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBProject + DBRole) -> BoxedFilter<(impl Reply,)> {
    let project = warp::path!("projects");
    let project_options = warp::path!("projects" / "options");
    let project_id = warp::path!("projects" / i32);

    let all_route = project
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and(warp::query::<QueryParams>())
        .and(warp::query::<PaginationParams>())
        .and_then(handlers::all_handler);
    
    let options = project_options
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and(warp::query::<QueryParams>())
        .and_then(handlers::options);

    let get_route = project_id
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::by_id);

    let create_route = project
        .and(with_github_auth())
        .and(warp::post())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_handler);

    let update_route = project_id
        .and(with_github_auth())
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(db_access.clone()))
        .and_then(handlers::update_handler);

    let delete_route = project_id
        .and(with_github_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_handler);

    all_route
        .or(create_route)
        .or(get_route)
        .or(update_route)
        .or(delete_route)
        .or(options)
        .boxed()
}
