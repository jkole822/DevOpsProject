use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::{
    extract::State,
    http::{StatusCode, header::HeaderMap},
    response::{IntoResponse, Json},
};
use chrono::{DateTime, Utc};
use redis::RedisResult;
use sqlx::Error;
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;
use crate::models::{
    DeleteUserInput, SessionRow, SessionUserIdRow, SessionUserRow, UserInput, UserRow,
};
use crate::redis_client::{Job, enqueue_job};

fn extract_bearer_token(headers: &HeaderMap) -> Result<String, (StatusCode, &'static str)> {
    let auth_header = headers
        .get("authorization")
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Unauthorized Request"))?;

    let auth_str = auth_header
        .to_str()
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid header value"))?;

    Ok(auth_str
        .strip_prefix("Bearer ")
        .unwrap_or(auth_str)
        .to_string())
}

pub async fn create_user(
    State(appState): State<Arc<AppState>>,
    Json(payload): Json<UserInput>,
) -> impl IntoResponse {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(payload.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Error hashing password").into_response();
        }
    };

    let user = match sqlx::query_as::<_, UserRow>("INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id, username, password, created_at")
        .bind(payload.username)
        .bind(password_hash)
        .fetch_one(&appState.pool)
        .await {
        Ok(user) => user,
        Err(Error::Database(db_err)) if db_err.constraint() == Some("users_username_key") => {
            return (StatusCode::BAD_REQUEST, "User already exists").into_response()
        },
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Error creating user").into_response(),
    };

    let now = Utc::now().timestamp();
    let ttl: i64 = now + 60 * 60 * 24 * 30;
    let expires_at: DateTime<Utc> = DateTime::from_timestamp(ttl, 0).unwrap();

    match sqlx::query_as::<_, SessionRow>("INSERT INTO sessions (token, user_id, expires_at) VALUES ($1, $2, $3) RETURNING id, token, user_id, created_at, expires_at")
        .bind(Uuid::new_v4())
        .bind(user.id)
        .bind(expires_at)
        .fetch_one(&appState.pool)
        .await
    {
        Ok(session) => Json(session).into_response(),
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    }
}

pub async fn delete_user(
    headers: HeaderMap,
    State(appState): State<Arc<AppState>>,
    Json(payload): Json<DeleteUserInput>,
) -> impl IntoResponse {
    let token = match extract_bearer_token(&headers) {
        Ok(t) => t,
        Err(err) => return err.into_response(),
    };

    if payload.confirmation.to_lowercase().trim() != "delete my account" {
        return (StatusCode::BAD_REQUEST, "Invalid confirmation").into_response();
    }

    let sessionUser = match sqlx::query_as::<_, SessionUserRow>("SELECT users.id, users.password FROM sessions INNER JOIN users ON sessions.user_id = users.id WHERE token = $1").bind(&token).fetch_one(&appState.pool).await {
        Ok(session) => session,
        Err(_) => return (StatusCode::BAD_REQUEST, "Unauthorized Request").into_response()
    };

    let parsed_hash = match PasswordHash::new(&sessionUser.password) {
        Ok(hash) => hash,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Invalid hash").into_response(),
    };

    if let Err(_) = Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash) {
        return (StatusCode::BAD_REQUEST, "Incorrect password").into_response();
    }

    if let Err(_) = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(sessionUser.id)
        .execute(&appState.pool)
        .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting user").into_response();
    }

    match sqlx::query("DELETE FROM sessions WHERE token = $1")
        .bind(token)
        .execute(&appState.pool)
        .await
    {
        Ok(_) => {
            let _: RedisResult<()> = enqueue_job(
                &appState.redis_client,
                Job {
                    job_type: "DELETE_USER_TASKS".to_string(),
                    task_id: None,
                    user_id: Some(sessionUser.id),
                },
            )
            .await;
            (StatusCode::OK, "Deleted user").into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting session").into_response(),
    }
}

pub async fn get_user(
    headers: HeaderMap,
    State(appState): State<Arc<AppState>>,
) -> impl IntoResponse {
    let token = match extract_bearer_token(&headers) {
        Ok(t) => t,
        Err(err) => return err.into_response(),
    };

    match sqlx::query_as::<_, SessionUserIdRow>("SELECT user_id FROM sessions WHERE token = $1")
        .bind(token)
        .fetch_one(&appState.pool)
        .await
    {
        Ok(session) => Json(session).into_response(),
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    }
}

pub async fn update_user(
    headers: HeaderMap,
    State(appState): State<Arc<AppState>>,
    Json(payload): Json<UserInput>,
) -> impl IntoResponse {
    let token = match extract_bearer_token(&headers) {
        Ok(t) => t,
        Err(err) => return err.into_response(),
    };

    let session = match sqlx::query_as::<_, SessionRow>("SELECT * FROM sessions WHERE token = $1")
        .bind(token)
        .fetch_one(&appState.pool)
        .await
    {
        Ok(row) => row,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Error updating user").into_response();
        }
    };

    match session.expires_at {
        Some(exp) if exp <= Utc::now() => {
            match sqlx::query("DELETE FROM sessions WHERE id = $1")
                .bind(session.id)
                .execute(&appState.pool)
                .await
            {
                Ok(_) => return (StatusCode::OK, "Session expired").into_response(),
                Err(_) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting session")
                        .into_response();
                }
            }
        }
        Some(_) => {}
        None => return (StatusCode::BAD_REQUEST, "Session token missing").into_response(),
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(payload.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Error hashing password").into_response();
        }
    };

    match sqlx::query("UPDATE users SET username = $1, password = $2 WHERE id = $3")
        .bind(payload.username)
        .bind(password_hash)
        .bind(session.user_id)
        .execute(&appState.pool)
        .await
    {
        Ok(_) => {
            match sqlx::query("DELETE FROM sessions WHERE id = $1")
                .bind(session.id)
                .execute(&appState.pool)
                .await
            {
                Ok(_) => (StatusCode::OK, "User updated").into_response(),
                Err(_) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting session")
                        .into_response();
                }
            }
        }
        Err(Error::Database(db_err)) if db_err.constraint() == Some("users_username_key") => {
            (StatusCode::BAD_REQUEST, "User already exists").into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error updating user").into_response(),
    }
}

pub async fn login(
    State(appState): State<Arc<AppState>>,
    Json(payload): Json<UserInput>,
) -> impl IntoResponse {
    let user = match sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE username = $1")
        .bind(payload.username)
        .fetch_one(&appState.pool)
        .await
    {
        Ok(user) => user,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid username or password").into_response(),
    };

    let parsed_hash = match PasswordHash::new(&user.password) {
        Ok(hash) => hash,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Invalid hash").into_response(),
    };

    if let Err(_) = Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash) {
        return (StatusCode::BAD_REQUEST, "Invalid username or password").into_response();
    }

    let now = Utc::now().timestamp();
    let ttl: i64 = now + 60 * 60 * 24 * 30;
    let expires_at: DateTime<Utc> = DateTime::from_timestamp(ttl, 0).unwrap();

    match sqlx::query_as::<_, SessionRow>(
        "INSERT INTO sessions (token, user_id, expires_at) VALUES ($1, $2, $3) RETURNING id, token, user_id, created_at, expires_at",
    )
        .bind(Uuid::new_v4())
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
