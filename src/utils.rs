use crate::{
    api::{health, issues, projects, repositories, users, roles, tasks, teams, subscriptions, notifications},
    db::{
        self,
        errors::DBError,
        pool::{DBAccess, DBAccessor},
    },
    errors::error_handler,
};
use ::warp::Reply;
use warp::{filters::BoxedFilter, http::Method, Filter};

pub async fn setup_db(url: &str) -> DBAccess {
    let db_pool = db::pool::create_db_pool(url)
        .map_err(DBError::DBPoolConnection)
        .expect("Failed to create DB pool");
    DBAccess::new(db_pool)
}

pub fn setup_filters(db: DBAccess) -> BoxedFilter<(impl Reply,)> {
    let health_route = health::routes::routes(db.clone());
    let projects_route = projects::routes::routes(db.clone());
    let repositories_route = repositories::routes::routes(db.clone());
    let issues_route = issues::routes::routes(db.clone());
    let users_route = users::routes::routes(db.clone());
    let teams_route = teams::routes::routes(db.clone());
    let roles_route = roles::routes::routes(db.clone());
    let tasks_route = tasks::routes::routes(db.clone());
    let subscriptions_route = subscriptions::routes::routes(db.clone());
    let notifications_route = notifications::routes::routes(db.clone());


    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Access-Control-Allow-Headers", "Access-Control-Request-Method", "Access-Control-Request-Headers", "Origin", "Accept", "X-Requested-With", "Content-Type", "Authorization"])
        .allow_credentials(true)
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE, Method::OPTIONS, Method::HEAD]);


    health_route
        .or(projects_route)
        .or(repositories_route)
        .or(issues_route)
        .or(users_route)
        .or(teams_route)
        .or(roles_route)
        .or(tasks_route)
        .or(subscriptions_route)
        .or(notifications_route)
        .recover(error_handler)
        .with(warp::log("api"))
        .with(cors)
        .boxed()
}

pub fn parse_ids(s: &str) -> Vec<i32> {
    let mut ids = Vec::new();
    for token in s.split_whitespace() {
        if let Ok(id) = token.parse::<i32>() {
            ids.push(id);
        }
    }
    ids
}

pub fn parse_comma_values(s: &str) -> Vec<String> {
    s.split(',').map(|el: &str| el.to_string()).collect()
}
