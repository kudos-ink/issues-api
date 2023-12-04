use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use super::db::DBContribution;
use super::handlers;

pub fn routes(db_access: impl DBContribution) -> BoxedFilter<(impl Reply, )> {
    let route = warp::path!("contribution")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || db_access.clone()))
        .and_then(handlers::create_contribution_handler);

        route.boxed()
}
