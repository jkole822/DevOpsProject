use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateSessionInput {
    pub username: String,
}

#[derive(FromRow, Serialize)]
pub struct SessionRow {
    pub id: Uuid,
    pub token: String,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(FromRow, Serialize)]
pub struct SessionUserRow {
    pub id: i32,
    pub password: String,
}

#[derive(FromRow, Serialize)]
pub struct SessionUserIdRow {
    pub user_id: i32,
}
