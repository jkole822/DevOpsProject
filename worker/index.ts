import Redis from "ioredis";
import { Pool } from "pg";
import dotenv from "dotenv";

dotenv.config();

const REDIS_URL = process.env.REDIS_URL || "redis://localhost:6379";
const DATABASE_URL = process.env.DATABASE_URL;

const redis = new Redis(REDIS_URL);
const pgPool = new Pool({ connectionString: DATABASE_URL });

async function completeTask(taskId: string) {
  console.log(`Completing task ${taskId}...`);
  const { rows } = await pgPool.query(
    `UPDATE tasks SET status = 'complete' WHERE id = $1 RETURNING *`,
    [taskId],
  );

  if (!rows.length) {
    console.log(`Task ${taskId} not found in DB`);
    return;
  }

  const task = rows[0];
  const cachedTasks = await redis.get("tasks");
  if (cachedTasks) {
    const tasks = JSON.parse(cachedTasks);
    const index = tasks.findIndex((t: { id: string }) => t.id === task.id);
    if (index >= 0) {
      tasks[index].status = "complete";
      await redis.set("tasks", JSON.stringify(tasks), "EX", 120);
    }
  }

  // Delete individual task cache so Task API will refresh it next time
  await redis.del(`task:${taskId}`);

  console.log(`Task ${taskId} marked as complete`);
}

async function deleteUserTasks(userId: string) {
  console.log(`Deleting tasks for user ${userId}...`);
  await pgPool.query("DELETE FROM tasks WHERE user_id = $1", [userId]);

  // const keys = await redis.keys(`tasks:${userId}*`);
  // if (keys.length > 0) {
  //   await redis.del(...keys);
  // }

  // Use SCAN instead of KEYS for production safety
  let cursor = "0";
  do {
    const [nextCursor, keys] = await redis.scan(cursor, "MATCH", `tasks:${userId}*`, "COUNT", 100);
    cursor = nextCursor;
    if (keys.length > 0) {
      await redis.del(...keys);
    }
  } while (cursor !== "0");

  console.log(`All tasks for user ${userId} deleted`);
}

let running = true;

async function processQueue() {
  console.log("Worker started, waiting for tasks...");
  while (running) {
    try {
      const result = await redis.brpop("JOB_QUEUE", 0);
      if (result) {
        const payload = JSON.parse(result[1]);
        switch (payload.type) {
          case "COMPLETE_TASK":
            await completeTask(payload.taskId);
            break;
          case "DELETE_USER_TASKS":
            await deleteUserTasks(payload.userId);
            break;
          default:
            console.warn("Unknown job type:", payload.type);
        }
      }
    } catch (err) {
      console.error("Error processing queue:", err);
      await new Promise((resolve) => setTimeout(resolve, 1000));
    }
  }

  console.log("Worker shutting down...");
  await pgPool.end();
  await redis.quit();
}

process.on("SIGINT", () => { running = false; });
process.on("SIGTERM", () => { running = false; });

processQueue();
