use std::convert::Infallible;

use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use super::db::DBContribution;
use super::handlers;

fn with_db(
    db_pool: impl DBContribution,
) -> impl Filter<Extract = (impl DBContribution,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBContribution) -> BoxedFilter<(impl Reply,)> {
    let contribution = warp::path!("contribution");
    let contribution_id = warp::path!("contribution" / i64);

    let get_contributions = contribution
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::get_contributions_handler);

    let get_contribution = contribution_id
        .and(warp::get())
        // .and(warp::path::param())
        .and(with_db(db_access.clone()))
        .and_then(handlers::get_contribution_handler);

    let create_contribution = contribution
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_contribution_handler);

    let delete_contribution = contribution_id
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_contribution_handler);

    let route = get_contributions
        .or(get_contribution)
        .or(create_contribution)
        .or(delete_contribution);

    route.boxed()
}
