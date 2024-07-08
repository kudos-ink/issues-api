use diesel::prelude::*;

use crate::db::{
    errors::DBError,
    pool::{DBAccess, DBAccessor},
};

use super::models::{Language, NewLanguage};
use crate::schema::languages::dsl as languages_dsl;

pub trait DBLanguage: Send + Sync + Clone + 'static {
    fn all(&self) -> Result<Vec<Language>, DBError>;
    fn create_or_get(&self, form: &NewLanguage) -> Result<Language, DBError>;
}

impl DBLanguage for DBAccess {
    fn all(&self) -> Result<Vec<Language>, DBError> {
        let conn = &mut self.get_db_conn();
        let result = languages_dsl::languages.load::<Language>(conn)?;
        Ok(result)
    }

    fn create_or_get(&self, form: &NewLanguage) -> Result<Language, DBError> {
        let conn = &mut self.get_db_conn();

        match languages_dsl::languages
            .filter(languages_dsl::slug.eq(form.slug.clone()))
            .first::<Language>(conn)
            .optional()?
        {
            Some(language) => Ok(language),
            None => {
                let language = diesel::insert_into(languages_dsl::languages)
                    .values(form)
                    .get_result(conn)
                    .map_err(DBError::from)?;

                Ok(language)
            }
        }
    }
}
