use std::convert::Infallible;

use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::api::repositories::db::DBRepository;
use crate::api::users::db::DBUser;
use crate::auth::with_auth;
use crate::types::PaginationParams;

use super::db::DBIssue;
use super::handlers;
use super::models::QueryParams;

fn with_db(
    db_pool: impl DBIssue + DBRepository + DBUser,
) -> impl Filter<Extract = (impl DBIssue + DBRepository + DBUser,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBIssue + DBRepository + DBUser) -> BoxedFilter<(impl Reply,)> {
    let issue = warp::path!("issues");
    let issue_id = warp::path!("issues" / i32);
    let issue_id_assignee = warp::path!("issues" / i32 / "assignee");

    let get_issues = issue
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and(warp::query::<QueryParams>())
        .and(warp::query::<PaginationParams>())
        .and_then(handlers::all_handler);

    let get_issue = issue_id
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::by_id);

    let create_issue = issue
        .and(with_auth())
        .and(warp::post())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_handler);

    let delete_issue = issue_id
        .and(with_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_handler);

    let update_issue = issue_id
        .and(with_auth())
        .and(warp::put())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::update_handler);

    let update_issue_assignee = issue_id_assignee
        .and(with_auth())
        .and(warp::patch())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::update_asignee_handler);

    let delete_issue_assignee = issue_id_assignee
        .and(with_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_asignee_handler);

    let route = get_issues
        .or(get_issue)
        .or(create_issue)
        .or(delete_issue)
        .or(update_issue)
        .or(update_issue_assignee)
        .or(delete_issue_assignee);

    route.boxed()
}
