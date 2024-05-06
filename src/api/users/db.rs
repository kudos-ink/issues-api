use diesel::dsl::now;
use diesel::prelude::*;

use super::models::{NewUser, UpdateUser, User};
use crate::schema::users::dsl as users_dsl;

use crate::db::{
    errors::DBError,
    pool::{DBAccess, DBAccessor},
};
use crate::types::PaginationParams;

pub trait DBUser: Send + Sync + Clone + 'static {
    fn by_id(&self, id: i32) -> Result<Option<User>, DBError>;
    fn by_username(&self, username: &str) -> Result<Option<User>, DBError>;
    fn all(&self, pagination: PaginationParams) -> Result<Vec<User>, DBError>;
    fn create(&self, user: &NewUser) -> Result<User, DBError>;
    fn update(&self, id: i32, user: &UpdateUser) -> Result<User, DBError>;
    fn delete(&self, id: i32) -> Result<(), DBError>;
}

impl DBUser for DBAccess {
    fn by_id(&self, id: i32) -> Result<Option<User>, DBError> {
        let conn = &mut self.get_db_conn();

        let result = users_dsl::users
            .find(id)
            .first::<User>(conn)
            .optional()
            .map_err(DBError::from)?;

        Ok(result)
    }
    fn by_username(&self, username: &str) -> Result<Option<User>, DBError> {
        let conn = &mut self.get_db_conn();
        let mut query = users_dsl::users.into_boxed();
        query = query.filter(users_dsl::username.eq(username));
        query = query.limit(1);
        let result: Vec<User> = query.load::<User>(conn)?;
        if result.is_empty() {
            Ok(None)
        } else {
            Ok(Some(User {
                id: result[0].id,
                username: result[0].username.clone(),
                created_at: result[0].created_at,
                updated_at: result[0].updated_at,
            }))
        }
    }

    fn all(&self, pagination: PaginationParams) -> Result<Vec<User>, DBError> {
        let conn = &mut self.get_db_conn();
        let mut query = users_dsl::users.into_boxed();

        query = query.offset(pagination.offset).limit(pagination.limit);

        let result = query.load::<User>(conn)?;
        Ok(result)
    }

    fn create(&self, user: &NewUser) -> Result<User, DBError> {
        let conn = &mut self.get_db_conn();

        let user = diesel::insert_into(users_dsl::users)
            .values(user)
            .get_result(conn)
            .map_err(DBError::from)?;

        Ok(user)
    }

    fn update(&self, id: i32, form: &UpdateUser) -> Result<User, DBError> {
        let conn = &mut self.get_db_conn();

        let user = diesel::update(users_dsl::users.filter(users_dsl::id.eq(id)))
            .set((form, users_dsl::updated_at.eq(now)))
            .get_result::<User>(conn)
            .map_err(DBError::from)?;

        Ok(user)
    }

    fn delete(&self, id: i32) -> Result<(), DBError> {
        let conn = &mut self.get_db_conn();
        diesel::delete(users_dsl::users.filter(users_dsl::id.eq(id)))
            .execute(conn)
            .map_err(DBError::from)?;

        Ok(())
    }
}
