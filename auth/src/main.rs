use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::{
    Router,
    extract::{Query, State},
    http::{StatusCode, header::HeaderMap},
    response::{IntoResponse, Json},
    routing::{delete, get, post, put},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;
use sqlx::{Error, FromRow, PgPool};
use std::collections::HashMap;
use tokio::net::TcpListener;
use uuid::Uuid;

#[derive(Deserialize)]
struct CreateSessionInput {
    user_id: i32,
}

#[derive(Deserialize)]
struct DeleteUserInput {
    confirmation: String,
    password: String,
}

#[derive(Deserialize)]
struct UserInput {
    username: String,
    password: String,
}

#[derive(FromRow, Serialize)]
struct SessionRow {
    id: Uuid,
    token: String,
    user_id: i32,
    created_at: DateTime<Utc>,
    expires_at: Option<DateTime<Utc>>,
}

#[derive(FromRow, Serialize)]
struct SessionUserRow {
    id: i32,
    password: String,
}

#[derive(FromRow, Serialize)]
struct SessionUserIdRow {
    user_id: i32,
}

#[derive(FromRow, Serialize)]
struct UserRow {
    id: i32,
    username: String,
    password: String,
    created_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL");
    let pool = PgPool::connect(&database_url).await.unwrap();
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    let app = Router::new()
        .route("/auth", post(create_user))
        .route("/auth", delete(delete_user))
        .route("/auth", get(get_user))
        .route("/auth", put(update_user))
        .route("/login", post(login))
        .route("/session", post(create_session))
        .with_state(pool);
    axum::serve(listener, app).await.unwrap();
}

async fn create_user(
    State(pool): State<PgPool>,
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
        .fetch_one(&pool)
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

    match sqlx::query("INSERT INTO sessions (token, user_id, expires_at) VALUES ($1, $2, $3)")
        .bind(Uuid::new_v4())
        .bind(user.id)
        .bind(expires_at)
        .execute(&pool)
        .await
    {
        Ok(_) => (StatusCode::OK, "User created").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error creating session").into_response(),
    }
}

async fn delete_user(
    headers: HeaderMap,
    State(pool): State<PgPool>,
    Json(payload): Json<DeleteUserInput>,
) -> impl IntoResponse {
    let auth_header = match headers.get("authorization") {
        Some(header) => header,
        None => return (StatusCode::BAD_REQUEST, "Unauthorized Request").into_response(),
    };

    let auth_string = match auth_header.to_str() {
        Ok(auth_str) => auth_str,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid header value").into_response(),
    };

    let token = auth_string.strip_prefix("Bearer ").unwrap_or(auth_string);

    if payload.confirmation.to_lowercase().trim() != "delete my account" {
        return (StatusCode::BAD_REQUEST, "Invalid confirmation").into_response();
    }

    let session = match sqlx::query_as::<_, SessionUserRow>("SELECT users.id, users.password FROM sessions INNER JOIN users ON sessions.user_id = users.id WHERE token = $1").bind(&token).fetch_one(&pool).await {
        Ok(session) => session,
        Err(_) => return (StatusCode::BAD_REQUEST, "Unauthorized Request").into_response()
    };

    let parsed_hash = match PasswordHash::new(&session.password) {
        Ok(hash) => hash,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Invalid hash").into_response(),
    };

    if let Err(_) = Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash) {
        return (StatusCode::BAD_REQUEST, "Incorrect password").into_response();
    }

    if let Err(_) = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(session.id)
        .execute(&pool)
        .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting user").into_response();
    }

    match sqlx::query("DELETE FROM sessions WHERE token = $1")
        .bind(token)
        .execute(&pool)
        .await
    {
        Ok(_) => (StatusCode::OK, "Deleted user").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting session").into_response(),
    }
}

async fn get_user(headers: HeaderMap, State(pool): State<PgPool>) -> impl IntoResponse {
    let auth_header = match headers.get("authorization") {
        Some(header) => header,
        None => return (StatusCode::BAD_REQUEST, "Unauthorized Request").into_response(),
    };

    let auth_string = match auth_header.to_str() {
        Ok(auth_str) => auth_str,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid header value").into_response(),
    };

    let token = auth_string.strip_prefix("Bearer ").unwrap_or(auth_string);

    match sqlx::query_as::<_, SessionUserIdRow>("SELECT user_id FROM sessions WHERE token = $1")
        .bind(token)
        .fetch_one(&pool)
        .await
    {
        Ok(session) => Json(session).into_response(),
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    }
}

async fn update_user(
    State(pool): State<PgPool>,
    Query(params): Query<HashMap<String, String>>,
    Json(payload): Json<UserInput>,
) -> impl IntoResponse {
    let token = match params.get("token") {
        Some(token) => token.to_string(),
        None => return (StatusCode::BAD_REQUEST, "Session token missing").into_response(),
    };

    let session = match sqlx::query_as::<_, SessionRow>("SELECT * FROM sessions WHERE token = $1")
        .bind(token)
        .fetch_one(&pool)
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
                .execute(&pool)
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
        .execute(&pool)
        .await
    {
        Ok(_) => {
            match sqlx::query("DELETE FROM sessions WHERE id = $1")
                .bind(session.id)
                .execute(&pool)
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

async fn login(State(pool): State<PgPool>, Json(payload): Json<UserInput>) -> impl IntoResponse {
    let user = match sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE username = $1")
        .bind(payload.username)
        .fetch_one(&pool)
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
        .bind(Uuid::new_v4().to_string())
        .bind(user.id)
        .bind(expires_at)
        .fetch_one(&pool)
        .await {
        Ok(session) => Json(session).into_response(),
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    }
}

async fn create_session(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateSessionInput>,
) -> impl IntoResponse {
    let now = Utc::now().timestamp();
    let ttl: i64 = now + 60 * 5;
    let expires_at: DateTime<Utc> = DateTime::from_timestamp(ttl, 0).unwrap();

    match sqlx::query_as::<_, SessionRow>(
        "INSERT INTO sessions (token, user_id, expires_at) VALUES ($1, $2, $3) RETURNING id, token, user_id, created_at, expires_at",
    )
    .bind(Uuid::new_v4().to_string())
    .bind(payload.user_id)
    .bind(expires_at)
    .fetch_one(&pool)
    .await {
        Ok(session) => Json(session).into_response(),
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    }
}
