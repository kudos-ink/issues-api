use std::convert::Infallible;

use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::api::roles::db::DBRole;
use crate::middlewares::github::auth::with_github_auth;

use super::db::DBNotification;
use super::handlers;

fn with_db(
    db_pool: impl DBNotification + DBRole,
) -> impl Filter<Extract = (impl DBNotification + DBRole,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBNotification + DBRole) -> BoxedFilter<(impl Reply,)> {
    let notifications = warp::path!("notifications");
    let notification_id = warp::path!("notifications" / i32);

    let get_notifications = notifications
        .and(warp::get())
        .and(with_github_auth())
        .and(with_db(db_access.clone()))
        .and_then(handlers::by_github_id);

    let delete_notification = notification_id
        .and(with_github_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_handler);

    let delete_all_notifications = notifications
        .and(with_github_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_all_handler);

    let route = get_notifications
        .or(delete_notification)
        .or(delete_all_notifications);

    route.boxed()
}
