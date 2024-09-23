use crate::{
    api::{health, issues, projects, repositories, users},
    db::{
        self,
        errors::DBError,
        pool::{DBAccess, DBAccessor},
    },
    errors::error_handler,
};
use ::warp::Reply;
use warp::{filters::BoxedFilter, Filter};

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

    health_route
        .or(projects_route)
        .or(repositories_route)
        .or(issues_route)
        .or(users_route)
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_headers(vec!["Authorization", "Content-Type"]),
        )
        .recover(error_handler)
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
