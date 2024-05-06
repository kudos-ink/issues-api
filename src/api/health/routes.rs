use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use super::db::DBHealth;
use super::handlers;

pub fn routes(db_access: impl DBHealth) -> BoxedFilter<(impl Reply,)> {
    let health_route = warp::path!("health")
        .and(warp::any().map(move || db_access.clone()))
        .and_then(handlers::health_handler)
        .boxed();

    health_route
}
