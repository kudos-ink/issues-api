use std::convert::Infallible;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::auth::with_auth;

use super::db::DBLanguage;
use super::handlers;

fn with_db(
    db_pool: impl DBLanguage,
) -> impl Filter<Extract = (impl DBLanguage,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBLanguage) -> BoxedFilter<(impl Reply,)> {
    let language = warp::path!("languages");

    let all_route = language
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::all_handler);

    let create_route = language
        .and(with_auth())
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_handler);

    all_route.or(create_route).boxed()
}
