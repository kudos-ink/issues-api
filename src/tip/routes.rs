use std::convert::Infallible;

use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::auth::with_auth;
use crate::types::TipId;
use crate::{issue::db::DBIssue, repository::db::DBRepository};

use super::db::DBTip;
use super::handlers;

fn with_db(
    db_pool: impl DBRepository + DBIssue + DBTip,
) -> impl Filter<Extract = (impl DBRepository + DBIssue + DBTip,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBRepository + DBIssue + DBTip) -> BoxedFilter<(impl Reply,)> {
    let tip = warp::path!("tip");
    let tip_id = warp::path!("tip" / TipId);

    let get_tip = tip_id
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::get_tip_handler);

    let create_tip = tip
        .and(with_auth())
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_tip_handler);

    let update_tip = tip_id
        .and(with_auth())
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(db_access.clone()))
        .and_then(handlers::update_tip_handler);

    let delete_tip = tip_id
        .and(with_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_tip_handler);

    let route = get_tip.or(create_tip).or(update_tip).or(delete_tip);

    route.boxed()
}
