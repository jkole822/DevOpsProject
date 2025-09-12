use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type RedisConn = Arc<Mutex<redis::aio::MultiplexedConnection>>;

#[derive(Serialize, Deserialize)]
pub struct Job {
    #[serde(rename = "type")]
    pub job_type: String,
    #[serde(rename = "taskId")]
    pub task_id: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
}

pub async fn connect_redis(url: &str) -> RedisConn {
    let client = redis::Client::open(url).expect("Failed to create Redis client");
    let conn = client
        .get_multiplexed_async_connection()
        .await
        .expect("Failed to connect to Redis");
    Arc::new(Mutex::new(conn))
}

pub async fn enqueue_job(redis_conn: &RedisConn, job: Job) -> redis::RedisResult<()> {
    let mut conn = redis_conn.lock().await;
    let job_json = serde_json::to_string(&job).unwrap();
    let _: isize = conn.rpush("JOB_QUEUE", job_json).await?;
    Ok(())
}
