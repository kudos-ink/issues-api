use bytes::Buf;
use warp::{
    http::StatusCode,
    reject,
    reject::Rejection,
    reply::{json, with_status, Reply},
};

use crate::{
    middlewares::github::model::GitHubUser,
};
use log::{error, info, warn};

use super::{
    db::DBUserSubscription,
    errors::UserSubscriptionError,
    models::{NewUserSubscription, DeleteUserSubscription},
};


pub async fn by_github_id(
     user: GitHubUser,
     db_access: impl DBUserSubscription) -> Result<impl Reply, Rejection> {
    let subscriptions = db_access.by_github_id(user.id)?;
    if subscriptions.is_empty() {
        Err(warp::reject::custom(UserSubscriptionError::NotFoundByGithubUser(user.id)))?
    } else {
        Ok(json(&subscriptions))
    }
}

pub async fn create_handler(
    user: GitHubUser,
    buf: impl Buf,
    db_access: impl DBUserSubscription,
) -> Result<impl Reply, Rejection> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let mut subscription: NewUserSubscription = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid subscription '{e}'");
        reject::custom(UserSubscriptionError::InvalidPayload(e))
    })?;
    validate_new_fields(&subscription)?;
    normalize_new_subscription(&mut subscription);
    subscription.github_id = Some(user.id);
    // TODO: validate that fields exist in the db
    create_subscription(db_access, subscription)
}

fn create_subscription(
    db_access: impl DBUserSubscription,
    subscription: NewUserSubscription,
) -> Result<warp::reply::WithStatus<warp::reply::Json>, Rejection> {

    match db_access.create(&subscription) {
        Ok(subscription) => {
            info!("subscription for user '{:?}' created", subscription.github_id);
            Ok(with_status(json(&subscription), StatusCode::CREATED))
        }
        Err(error) => {
            error!("error creating the subscription '{:?}': {}", subscription, error);
            let error_str = error.to_string();
            if error_str.contains("user_subscriptions_github_id_purpose_key") || error_str.contains("user_subscriptions_github_id_stack_level_key") ||  error_str.contains("user_subscriptions_github_id_technology_key") {
                Err(warp::reject::custom(UserSubscriptionError::AlreadyExists(
                    subscription.github_id.unwrap_or(0) as i32
                )))
            } else {
                Err(warp::reject::custom(UserSubscriptionError::CannotCreate(
                    "error creating the subscription".to_string(),
                )))
            }
        }
    }
}


pub async fn delete_handler(
    user: GitHubUser,
    buf: impl Buf,
    db_access: impl DBUserSubscription ,
) -> Result<impl Reply, Rejection> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let mut subscription: DeleteUserSubscription = serde_path_to_error::deserialize(des).map_err(|e| {
        let e = e.to_string();
        warn!("invalid subscription '{e}'");
        reject::custom(UserSubscriptionError::InvalidPayload(e))
    })?;
    validate_delete_fields(&subscription)?;
    normalize_delete_subscription(&mut subscription);
    subscription.github_id = Some(user.id);
    delete_subscription(db_access, subscription)
}

fn validate_new_fields(subscription: &NewUserSubscription) -> Result<(), Rejection> {
    let fields = [
        ("purpose", subscription.purpose.is_some()),
        ("stack level", subscription.stack_level.is_some()),
        ("technology", subscription.technology.is_some()),
    ];
    let provided_fields: Vec<_> = fields.iter()
        .filter(|(_, is_some)| *is_some)
        .map(|(name, _)| *name)
        .collect();
    

    if provided_fields.len() > 1 {
        return Err(warp::reject::custom(UserSubscriptionError::InvalidPayload(
            format!("only one field can be provided at a time, got: {}", provided_fields.join(", "))
        )));
    }
    Ok(())
}

fn validate_delete_fields(subscription: &DeleteUserSubscription) -> Result<(), Rejection> {
    let fields = [
        ("purpose", subscription.purpose.is_some()),
        ("stack level", subscription.stack_level.is_some()),
        ("technology", subscription.technology.is_some()),
    ];
    let provided_fields: Vec<_> = fields.iter()
        .filter(|(_, is_some)| *is_some)
        .map(|(name, _)| *name)
        .collect();
    

    if provided_fields.len() > 1 {
        return Err(warp::reject::custom(UserSubscriptionError::InvalidPayload(
            format!("only one field can be provided at a time, got: {}", provided_fields.join(", "))
        )));
    }
    Ok(())
}

fn delete_subscription(
    db_access: impl DBUserSubscription,
    subscription: DeleteUserSubscription,
) -> Result<warp::reply::WithStatus<warp::reply::Json>, Rejection> {
    match db_access.delete(&subscription) {
        Ok(_) => {
            info!("subscription for user '{:?}' deleted", subscription.github_id);
            Ok(with_status(json(&subscription), StatusCode::NO_CONTENT))
        }
        Err(error) => {
            error!("error deleting the subscription '{:?}': {}", subscription, error);
            Err(warp::reject::custom(UserSubscriptionError::CannotDelete(
                "error deleting the subscription".to_string(),
            )))
        }
    }
}

fn normalize_new_subscription(subscription: &mut NewUserSubscription)  {
    if subscription.purpose.is_some() {
        subscription.purpose = Some(subscription.purpose.as_ref().unwrap().to_lowercase());
    }
    if subscription.stack_level.is_some() {
        subscription.stack_level = Some(subscription.stack_level.as_ref().unwrap().to_lowercase());
    }
    if subscription.technology.is_some() {
        subscription.technology = Some(subscription.technology.as_ref().unwrap().to_lowercase());
    }
}
fn normalize_delete_subscription(subscription: &mut DeleteUserSubscription)  {
    if subscription.purpose.is_some() {
        subscription.purpose = Some(subscription.purpose.as_ref().unwrap().to_lowercase());
    }
    if subscription.stack_level.is_some() {
        subscription.stack_level = Some(subscription.stack_level.as_ref().unwrap().to_lowercase());
    }
    if subscription.technology.is_some() {
        subscription.technology = Some(subscription.technology.as_ref().unwrap().to_lowercase());
    }
}