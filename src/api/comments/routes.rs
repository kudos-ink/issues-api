use std::convert::Infallible;
use warp::{Filter, Reply, filters::BoxedFilter};
use crate::middlewares::github::auth::with_github_auth;
use crate::api::users::db::DBUser;
use super::db::DBComment;
use super::handlers;

fn with_db(db_pool: impl DBComment + DBUser) -> impl Filter<Extract = (impl DBComment + DBUser,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBComment + DBUser) -> BoxedFilter<(impl Reply,)> {
    let base = warp::path!("tasks" / i32 / "comments");

    let get_comments = base
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::get_comments_handler);

    let create_comment = base
        .and(warp::post())
        .and(with_github_auth())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_comment_handler);
    
    get_comments.or(create_comment).boxed()
}