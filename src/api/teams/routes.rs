use std::convert::Infallible;

use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use super::db::{DBTeam, DBTeamMembership};
use super::handlers;
use crate::auth::with_auth;


fn with_db(
    db_pool: impl DBTeam + DBTeamMembership,
) -> impl Filter<Extract = (impl DBTeam + DBTeamMembership,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn routes(db_access: impl DBTeam + DBTeamMembership) -> BoxedFilter<(impl Reply,)> {
    let teams = warp::path!("teams");
    let team_id = warp::path!("teams" / i32);
    let team_members = warp::path!("teams" / i32 / "members");
    let member_id = warp::path!("teams" / i32 / "members" / i32);

    let get_teams = teams
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::get_all_teams);

    let get_team = team_id
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::get_team_by_id);

    let create_team = teams
        .and(with_auth())
        .and(warp::post())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::create_team);

    let update_team = team_id
        .and(with_auth())
        .and(warp::put())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::update_team);

    let delete_team = team_id
        .and(with_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::delete_team);

    let add_member = team_members
        .and(with_auth())
        .and(warp::post())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::add_member_to_team);

    let list_members = team_members
        .and(warp::get())
        .and(with_db(db_access.clone()))
        .and_then(handlers::list_team_members);

    let update_member_role = member_id
        .and(with_auth())
        .and(warp::patch())
        .and(warp::body::aggregate())
        .and(with_db(db_access.clone()))
        .and_then(handlers::update_member_role);

    let remove_member = member_id
        .and(with_auth())
        .and(warp::delete())
        .and(with_db(db_access.clone()))
        .and_then(handlers::remove_member_from_team);

    let route = get_teams
        .or(get_team)
        .or(create_team)
        .or(update_team)
        .or(delete_team)
        .or(add_member)
        .or(list_members)
        .or(update_member_role)
        .or(remove_member);

    route.boxed()
}
