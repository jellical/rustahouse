use crate::db::diesel::DatabaseConnection;
use crate::db::models::users::User;
use crate::db::schema::users;
use crate::error::internal_error;
use axum::http::StatusCode;
use axum::Json;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub async fn list_users(DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let res = users::table
        .select(User::as_select())
        .load(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}
