use ::warp::Reply;
use std::env;
use warp::{filters::BoxedFilter, Filter};

use crate::{
    db::{
        pool::{DBAccess, DBAccessor},
        utils::init_db,
    },
    handlers, health, issue, organization, repository, tip, user,
};

pub async fn setup_db(url: &String) -> DBAccess {
    let db = init_db(url.to_string())
        .await
        .expect("Failed to initialize the database");

    if let Ok(db_file) = env::var("DATABASE_INIT_FILE") {
        db.init_db(&db_file)
            .await
            .expect("Failed to apply database migrations");
    }

    db
}

pub fn setup_filters(db: DBAccess) -> BoxedFilter<(impl Reply,)> {
    let health_route = health::routes::routes(db.clone());
    let users_route = user::routes::routes(db.clone());
    let organizations_route = organization::routes::routes(db.clone());
    let repositories_route = repository::routes::routes(db.clone());
    let tips_route = tip::routes::routes(db.clone());
    let issues_route = issue::routes::routes(db.clone());

    let error_handler = handlers::error_handler;

    health_route
        .or(users_route)
        .or(organizations_route)
        .or(repositories_route)
        .or(tips_route)
        .or(issues_route)
        .with(warp::cors().allow_any_origin())
        .recover(error_handler)
        .boxed()
}
