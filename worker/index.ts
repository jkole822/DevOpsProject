import Redis from "ioredis";
import { Pool } from "pg";
import dotenv from "dotenv";

dotenv.config();

const REDIS_URL = process.env.REDIS_URL || "redis://localhost:6379";
const DATABASE_URL = process.env.DATABASE_URL;

const redis = new Redis(REDIS_URL);
const pgPool = new Pool({ connectionString: DATABASE_URL });

const TASK_QUEUE = "task_queue";

async function completeTask(taskId: string) {
  console.log(`Completing task ${taskId}...`);
  const { rows } = await pgPool.query(
    `UPDATE tasks SET status = 'complete' WHERE id = $1 RETURNING *`,
    [taskId]
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

async function processQueue() {
  console.log("Worker started, waiting for tasks...");
  while (true) {
    try {
      // BRPOP blocks until an item is available in task_queue
      const result = await redis.brpop(TASK_QUEUE, 0); // 0 = block indefinitely

      if (result) {
        const taskId = result[1]; // Get value from result = [queue_name, value]
        await completeTask(taskId);
      }
    } catch (err) {
      console.error("Error processing queue:", err);
      await new Promise((resolve) => setTimeout(resolve, 1000));
    }
  }
}

processQueue();
