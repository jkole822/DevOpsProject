use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct UserInput {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct DeleteUserInput {
    pub confirmation: String,
    pub password: String,
}

#[derive(FromRow, Serialize)]
pub struct UserRow {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow, Serialize)]
pub struct UserLimitedRow {
    pub id: i32,
    pub username: String,
}
