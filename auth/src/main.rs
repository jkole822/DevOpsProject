mod controllers;
mod models;
mod redis_client;

use axum::{
    Router,
    http::Method,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use crate::controllers::{create_session, create_user, delete_user, get_user, login, update_user};
use crate::redis_client::{RedisConn, connect_redis};

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    redis_client: RedisConn,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL");
    let redis_url = std::env::var("REDIS_URL").expect("Missing REDIS_URL");
    let pool = PgPool::connect(&database_url).await.unwrap();
    let redis_client = connect_redis(&redis_url).await;
    let state = Arc::new(AppState { redis_client, pool });
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        .route("/auth", post(create_user))
        .route("/auth", delete(delete_user))
        .route("/auth", get(get_user))
        .route("/auth", put(update_user))
        .route("/login", post(login))
        .route("/session", post(create_session))
        .with_state(state)
        .layer(cors);

    axum::serve(listener, app).await.unwrap();
}
