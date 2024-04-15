use std::convert::Infallible;

use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::auth::with_auth;
use crate::http::{GetPagination, GetSort};

use super::db::DBUser;
use super::handlers;
use super::models::GetUserQuery;

fn with_db(
    db_pool: impl DBUser,
) -> impl Filter<Extract = (impl DBUser,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBUser) -> BoxedFilter<(impl Reply,)> {
    let user = warp::path!("users");
    let user_id = warp::path!("users" / i32);
    let user_name = warp::path!("users" / String);

    let get_users = user
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and(warp::query::<GetUserQuery>())
        .and(warp::query::<GetPagination>())
        .and(warp::query::<GetSort>())
        .and_then(handlers::get_users_handler);

    let get_user = user_id
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and(warp::query::<GetUserQuery>())
        .and_then(handlers::get_user_handler);

    let get_user_by_name = user_name
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and(warp::query::<GetUserQuery>())
        .and_then(handlers::get_user_by_name_handler);
    // TODO: add maintainers
    let create_user = user
        .and(with_auth())
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_user_handler);
    // TODO: add PATCH maintainers
    let delete_user = user_id
        .and(with_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_user_handler);

    let route = get_users
        .or(create_user)
        .or(get_user)
        .or(delete_user)
        .or(get_user_by_name);

    route.boxed()
}
