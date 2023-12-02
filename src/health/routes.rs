use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::db::{types::DBPool, utils::with_db};

use super::handlers;


pub fn routes(db_pool: DBPool) -> BoxedFilter<(impl Reply, )> {

    let health_route = warp::path!("health")
        .and(with_db(db_pool.clone()))
        .and_then(handlers::health_handler);

    health_route.boxed()
}
