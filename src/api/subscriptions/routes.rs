use std::convert::Infallible;

use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::api::roles::db::DBRole;
use crate::middlewares::github::auth::with_github_auth;

use super::db::DBUserSubscription;
use super::handlers;

fn with_db(
    db_pool: impl DBUserSubscription + DBRole,
) -> impl Filter<Extract = (impl DBUserSubscription + DBRole,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBUserSubscription + DBRole) -> BoxedFilter<(impl Reply,)> {
    let subscriptions = warp::path!("subscriptions");

    let get_user_subscriptions = subscriptions
        .and(warp::get())
        .and(with_github_auth())
        .and(with_db(db_access.clone()))
        .and_then(handlers::by_github_id);

    let create_subscription = subscriptions
        .and(with_github_auth())
        .and(warp::post())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_handler);

    let delete_subscription = subscriptions
        .and(with_github_auth())
        .and(warp::delete())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_handler);

    let route = get_user_subscriptions
        .or(create_subscription)
        .or(delete_subscription);

    route.boxed()
}
