use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable,Insertable)]
#[diesel(table_name = crate::schema::student)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize,Deserialize)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub subjects: Option<Vec<Option<String>>>,
}