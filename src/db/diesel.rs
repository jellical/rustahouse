use crate::error::internal_error;
use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection,
};
use dotenvy::dotenv;
use std::env;

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub async fn get_connection_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

    pool
}


pub(crate) struct DatabaseConnection(
    pub bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    S: Send + Sync,
    Pool: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = Pool::from_ref(state);

        let conn = pool.get_owned().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}


