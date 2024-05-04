use crate::schema::languages;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = languages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Language {
    pub id: i32,
    pub name: String,
}
