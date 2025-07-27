use std::convert::Infallible;
use warp::{Filter, Reply, filters::BoxedFilter};
use crate::middlewares::github::auth::with_github_auth;
use crate::api::users::db::DBUser;
use crate::api::roles::db::DBRole;
use super::db::DBComment;
use super::handlers;

fn with_db(db_pool: impl DBComment + DBUser + DBRole) -> impl Filter<Extract = (impl DBComment + DBUser + DBRole,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBComment + DBUser + DBRole) -> BoxedFilter<(impl Reply,)> {
    let base = warp::path!("tasks" / i32 / "comments");
    let comment_id = warp::path!("comments" / i32);

    let get_comments = base
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::get_comments_handler);

    let get_comment_by_id = base
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::by_comment_id_handler);

    let create_comment = base
        .and(with_github_auth())
        .and(warp::post())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_comment_handler);

    let delete_comment = comment_id
        .and(with_github_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_comment_handler);
    
    get_comments
    .or(get_comment_by_id)
    .or(create_comment)
    .or(delete_comment)
    .boxed()
}