mod db;
mod error;
mod modules;

use crate::db::diesel::get_connection_pool;
use crate::modules::users::controller::list_users;
use axum::{
    routing::get, Router,
};

#[tokio::main]
async fn main() {
    let pool = get_connection_pool().await;


    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(list_users))
        .with_state(pool);

    println!("Something_happening");
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}



