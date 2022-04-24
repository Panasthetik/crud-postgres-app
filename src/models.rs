
/* Import macros and others */
use crate::schema::*;

/* For beeing able to serialize */
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Profile {
    pub id: i32, 
    pub alias: String,
    pub full_name: Option<String>,
    pub photo: String,
    pub mood: i32,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name="profiles_db"]
pub struct NewProfile<'x> {
    pub alias: &'x str,
    pub full_name: Option<&'x str>,
    pub photo: String,
    pub mood: i32,
}