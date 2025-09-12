use crate::AppState;
use crate::models::{CreateSessionInput, SessionRow, UserLimitedRow};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_session(
    State(appState): State<Arc<AppState>>,
    Json(payload): Json<CreateSessionInput>,
) -> impl IntoResponse {
    let now = Utc::now().timestamp();
    let ttl: i64 = now + 60 * 5;
    let expires_at: DateTime<Utc> = DateTime::from_timestamp(ttl, 0).unwrap();

    let user = match sqlx::query_as::<_, UserLimitedRow>(
        "SELECT username, id FROM users WHERE username = $1",
    )
    .bind(payload.username)
    .fetch_one(&appState.pool)
    .await
    {
        Ok(user) => user,
        Err(_) => return (StatusCode::BAD_REQUEST, "User not found").into_response(),
    };

    match sqlx::query_as::<_, SessionRow>(
        "INSERT INTO sessions (token, user_id, expires_at) VALUES ($1, $2, $3) RETURNING id, token, user_id, created_at, expires_at",
    )
        .bind(Uuid::new_v4().to_string())
        .bind(user.id)
        .bind(expires_at)
        .fetch_one(&appState.pool)
        .await {
        Ok(session) => Json(session).into_response(),
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    }
}
