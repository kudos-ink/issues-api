use std::convert::Infallible;

use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::auth::with_auth;
use crate::types::PaginationParams;

use super::db::DBUser;
use super::handlers;
use super::models::QueryParams;

fn with_db(
    db_pool: impl DBUser,
) -> impl Filter<Extract = (impl DBUser,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBUser) -> BoxedFilter<(impl Reply,)> {
    let user = warp::path!("users");
    let user_id = warp::path!("users" / i32);
    let user_username = warp::path!("users" / "username" / String);

    let get_users = user
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and(warp::query::<QueryParams>())
        .and(warp::query::<PaginationParams>())
        .and_then(handlers::all_handler);

    let get_user = user_id
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::by_id);
    
    let get_user_by_username = user_username
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::by_username);

    let create_user = user
        .and(with_auth())
        .and(warp::post())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_handler);

    let update_user = user_id
        .and(with_auth())
        .and(warp::put())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::update_handler);


    let delete_user = user_id
        .and(with_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_handler);

    let route = get_users
        .or(create_user)
        .or(get_user)
        .or(get_user_by_username)
        .or(delete_user)
        .or(update_user);

    route.boxed()
}
