use std::convert::Infallible;

use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::api::projects::db::DBProject;
use crate::api::users::db::DBUser;
use crate::middlewares::github::auth::with_github_auth;
use crate::types::PaginationParams;

use super::db::DBRole;
use super::handlers;

fn with_db(
    db_pool: impl DBRole + DBUser + DBProject,
) -> impl Filter<Extract = (impl DBRole + DBUser + DBProject,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBRole + DBUser + DBProject) -> BoxedFilter<(impl Reply,)> {
    let role = warp::path!("roles");
    let role_id = warp::path!("roles" / i32);
    let role_assignation = warp::path!("roles" /"assignation");
    let role_assignation_id = warp::path!("roles" /"assignation"/ i32);

    let get_roles = role
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and(warp::query::<PaginationParams>())
        .and_then(handlers::all_handler);

    let get_role = role_id
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::by_id);

    let create_role = role
        .and(with_github_auth())
        .and(warp::post())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_handler);

    let delete_role = role_id
        .and(with_github_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_handler);

    let update_role = role_id
        .and(with_github_auth())
        .and(warp::put())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::update_handler);

    let create_role_assignation = role_assignation
        .and(with_github_auth())
        .and(warp::post())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_role_to_user_and_project);

    let delete_role_assignation = role_assignation_id
        .and(with_github_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_role_to_user_and_project);

    let route = get_roles
        .or(get_role)
        .or(create_role)
        .or(delete_role)
        .or(update_role)
        .or(create_role_assignation)
        .or(delete_role_assignation);

    route.boxed()
}
