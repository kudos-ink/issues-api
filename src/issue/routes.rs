use std::convert::Infallible;

use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::auth::with_auth;
use crate::organization::db::DBOrganization;
use crate::repository::db::DBRepository;
use crate::tip::db::DBTip;
use crate::tip::handlers::get_tip_by_issue_handler;
use crate::types::IssueId;

use super::db::DBIssue;
use super::handlers;

fn with_db(
    db_pool: impl DBIssue + DBOrganization + DBRepository,
) -> impl Filter<Extract = (impl DBIssue + DBOrganization + DBRepository,), Error = Infallible> + Clone
{
    warp::any().map(move || db_pool.clone())
}

fn with_db_tip(
    db_pool: impl DBTip,
) -> impl Filter<Extract = (impl DBTip,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(
    db_access: impl DBIssue + DBOrganization + DBRepository + DBTip + Clone,
) -> BoxedFilter<(impl Reply,)> {
    let issue = warp::path!("issues");
    let issue_id = warp::path!("issues" / i32);

    let get_issues = issue
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::get_issues_handler);

    let get_issue = issue_id
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::get_issue_handler);

    let create_issue = issue
        .and(with_auth())
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_issue_handler);

    let delete_issue = issue_id
        .and(with_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_issue_handler);

    let get_tip_by_issue = warp::path!("issues" / IssueId / "tip")
        .and(warp::get())
        .and(with_db_tip(db_access.clone())) // Use the DBTip filter
        .and_then(get_tip_by_issue_handler);

    let route = get_issues
        .or(get_issue)
        .or(create_issue)
        .or(delete_issue)
        .or(get_tip_by_issue);

    route.boxed()
}
