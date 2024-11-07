use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, Insertable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

pub struct CreateNewUser {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}