use diesel::prelude::*;

use super::models::{NewUserSubscription, DeleteUserSubscription, UserSubscription};
use crate::schema::user_subscriptions::dsl as subscriptions_dsl;

use crate::db::{
    errors::DBError,
    pool::{DBAccess, DBAccessor},
};

pub trait DBUserSubscription: Send + Sync + Clone + 'static {
    fn by_github_id(&self, github_id: i64) -> Result<Vec<UserSubscription>, DBError>;
    fn create(&self, subscription: &NewUserSubscription) -> Result<UserSubscription, DBError>;
    fn delete(&self, subscription: &DeleteUserSubscription) -> Result<(), DBError>;
}

impl DBUserSubscription for DBAccess {
    fn by_github_id(&self, github_id: i64) -> Result<Vec<UserSubscription>, DBError> {
        let conn = &mut self.get_db_conn();

        let result = subscriptions_dsl::user_subscriptions
            .filter(subscriptions_dsl::github_id.eq(github_id))
            .load::<UserSubscription>(conn)
            .map_err(DBError::from)?;

        Ok(result)
    }

    fn create(&self, subscription: &NewUserSubscription) -> Result<UserSubscription, DBError> {
        let conn = &mut self.get_db_conn();

        let subscription = diesel::insert_into(subscriptions_dsl::user_subscriptions)
            .values(subscription)
            .get_result(conn)
            .map_err(DBError::from)?;

        Ok(subscription)
    }

    fn delete(&self, subscription: &DeleteUserSubscription) -> Result<(), DBError> {
        let conn = &mut self.get_db_conn();
        let github_id = subscription.github_id.ok_or_else(|| {
            DBError::DBQuery(diesel::result::Error::NotFound)
        })?;
        
        let query = subscriptions_dsl::user_subscriptions
            .filter(subscriptions_dsl::github_id.eq(github_id));

        match (&subscription.purpose, &subscription.stack_level, &subscription.technology) {
            (Some(purpose), None, None) => {
                diesel::delete(
                    query.filter(subscriptions_dsl::purpose.eq(purpose)))
                    .execute(conn)
                    .map_err(DBError::from)?;
            }
            (None, Some(stack_level), None) => {
                diesel::delete(
                    query.filter(subscriptions_dsl::stack_level.eq(stack_level)))
                    .execute(conn)
                    .map_err(DBError::from)?;
            }
            (None, None, Some(technology)) => {
                diesel::delete(
                    query.filter(subscriptions_dsl::technology.eq(technology)))
                    .execute(conn)
                    .map_err(DBError::from)?;
            }
            _ => return Err(DBError::DBQuery(diesel::result::Error::NotFound)),
        }

        Ok(())
    }
}