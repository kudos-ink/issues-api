use std::convert::Infallible;

use crate::api::roles::db::DBRole;
use crate::api::users::db::DBUser;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};
use crate::middlewares::github::auth::with_github_auth;
use crate::types::PaginationParams;

use super::db::DBTask;
use super::handlers;
use super::models::QueryParams;

fn with_db(
    db_pool: impl DBTask + DBUser + DBRole,
) -> impl Filter<Extract = (impl DBTask + DBUser + DBRole,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBTask + DBUser + DBRole) -> BoxedFilter<(impl Reply,)> {
    let task = warp::path!("tasks");
    let task_id = warp::path!("tasks" / i32);
    let task_upvote = warp::path!("tasks" / "upvotes");
    let task_downvote = warp::path!("tasks" / "downvotes");
    let task_vote_id = warp::path!("tasks" / "votes" / i32); // TODO:

    let get_tasks = task
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and(warp::query::<QueryParams>())
        .and(warp::query::<PaginationParams>())
        .and_then(handlers::all_handler);

    let get_task = task_id
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::by_id);

    let create_task = task
        .and(with_github_auth())
        .and(warp::post())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_handler);

    let delete_task = task_id
        .and(with_github_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_handler);

    let update_task = task_id
        .and(with_github_auth())
        .and(warp::put())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::update_handler);

    let create_task_upvote = task_upvote
        .and(with_github_auth())
        .and(warp::post())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::add_upvote_to_task);

    let create_task_downvote = task_downvote
        .and(with_github_auth())
        .and(warp::post())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::add_downvote_to_task);
    
    let delete_task_vote = task_vote_id
        .and(with_github_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_task_vote);

    let route = get_tasks
        .or(get_task)
        .or(create_task)
        .or(delete_task)
        .or(update_task)
        .or(create_task_upvote)
        .or(create_task_downvote)
        .or(delete_task_vote);

    route.boxed()
}
