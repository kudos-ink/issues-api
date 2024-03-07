use std::convert::Infallible;

use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::auth::with_auth;

use super::db::DBUser;
use super::handlers;

fn with_db(
    db_pool: impl DBUser,
) -> impl Filter<Extract = (impl DBUser,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBUser) -> BoxedFilter<(impl Reply,)> {
    let user = warp::path!("users");
    let user_id = warp::path!("users" / i32);

    let get_users = user
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::get_users_handler);

    let get_user = user_id
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::get_user_handler);

    let create_user = user
        .and(with_auth())
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_user_handler);

    let delete_user = user_id
        .and(with_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_user_handler);

    let route = get_users.or(get_user).or(create_user).or(delete_user);

    route.boxed()
}
