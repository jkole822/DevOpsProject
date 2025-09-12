from contextlib import asynccontextmanager
from datetime import datetime
from dotenv import load_dotenv
from fastapi import Depends, FastAPI, Header, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import List
import asyncpg
import httpx
import json
import os
import redis.asyncio as redis

load_dotenv()

AUTH_URL = os.getenv("AUTH_URL")
DATABASE_URL = os.getenv("DATABASE_URL")
CACHE_TTL = 120

@asynccontextmanager
async def lifespan(app: FastAPI):
    app.state.db_pool = await asyncpg.create_pool(DATABASE_URL)
    app.state.redis = redis.from_url(os.getenv("REDIS_URL", "redis://redis:6379/0"), decode_responses=True)
    await app.state.redis.ping()

    yield

    await app.state.db_pool.close()
    await app.state.redis.close()

app = FastAPI(lifespan=lifespan)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["GET", "POST", "PUT", "DELETE"],
    allow_headers=["*"],
)

# --- Models ---
class Task(BaseModel):
    id: int
    title: str
    done: bool = False
    status: str
    user_id: int
    created_at: datetime

class TaskCreate(BaseModel):
    title: str

class TaskUpdate(BaseModel):
    title: str
    done: bool

# --- Helpers ---
async def get_user_from_auth(authorization: str = Header(...)):
    if not authorization.startswith("Bearer "):
        raise HTTPException(status_code=401, detail="Invalid auth header")
    token = authorization.removeprefix("Bearer ").strip()

    async with httpx.AsyncClient() as client:
        resp = await client.get(f"{AUTH_URL}/auth", headers={"Authorization": f"Bearer {token}"})
        if resp.status_code != 200:
            raise HTTPException(status_code=401, detail="Invalid user")
        return resp.json()

# --- Routes ---
@app.get("/health")
async def health():
    try:
        async with app.state.db_pool.acquire() as conn:
            await conn.execute("SELECT 1")

        pong = await app.state.redis.ping()

        return {
            "status": "ok",
            "postgres": "ok",
            "redis": "ok" if pong else "down"
        }

    except Exception as e:
        return {
            "status": "error",
            "details": str(e)
        }

@app.get("/tasks", response_model=List[Task])
async def list_tasks(user=Depends(get_user_from_auth)):
    cache_key = f"tasks:{user['user_id']}"
    cached = await app.state.redis.get(cache_key)
    if cached:
        tasks_data = json.loads(cached)
        return [Task(**t) for t in tasks_data]

    async with app.state.db_pool.acquire() as conn:
        rows = await conn.fetch("SELECT * FROM tasks WHERE user_id = $1", user["user_id"])

    tasks_data = [dict(row) for row in rows]
    for task in tasks_data:
        task["created_at"] = task["created_at"].isoformat()

    await app.state.redis.set(cache_key, json.dumps(tasks_data), ex=CACHE_TTL)

    return [Task(**t) for t in tasks_data]

@app.get("/tasks/{task_id}", response_model=Task)
async def get_task(task_id: int, user=Depends(get_user_from_auth)):
    cache_key = f"task:{user['user_id']}:{task_id}"
    cached = await app.state.redis.get(cache_key)
    if cached:
        return Task(**json.loads(cached))

    async with app.state.db_pool.acquire() as conn:
        row = await conn.fetchrow("SELECT * FROM tasks WHERE id = $1 AND user_id = $2", task_id, user["user_id"])
        if not row:
            raise HTTPException(status_code=404, detail="Task not found")

    task_data = dict(row)
    task_data["created_at"] = task_data["created_at"].isoformat()

    await app.state.redis.set(cache_key, json.dumps(task_data), ex=CACHE_TTL)

    return Task(**task_data)

@app.post("/tasks", response_model=Task)
async def create_task(task: TaskCreate, user=Depends(get_user_from_auth)):
    async with app.state.db_pool.acquire() as conn:
        row = await conn.fetchrow(
            """
            INSERT INTO tasks (title, status, user_id)
            VALUES ($1, $2, $3)
            RETURNING id, title, done, status, user_id, created_at 
            """,
            task.title, "queued", user["user_id"]
        )
        task_id = row["id"]

    await app.state.redis.delete(f"tasks:{user['user_id']}")
    await app.state.redis.rpush(
        "JOB_QUEUE",
        json.dumps({
            "type": "COMPLETE_TASK",
            "taskId": str(task_id)
        })
    )

    return Task(**dict(row))

@app.put("/tasks/{task_id}", response_model=Task)
async def update_task(task_id: int, task: TaskUpdate, user=Depends(get_user_from_auth)):
    async with app.state.db_pool.acquire() as conn:
        row = await conn.fetchrow(
            """
            UPDATE tasks SET title = $1, done = $2, status = $3
            WHERE id = $4 AND user_id = $5     
            RETURNING id, title, done, status, user_id, created_at 
            """,
            task.title, task.done, "queued", task_id, user["user_id"]
        )
        if not row:
            raise HTTPException(status_code=404, detail="Task not found")

    await app.state.redis.delete(
        f"task:{user['user_id']}:{task_id}",
        f"tasks:{user['user_id']}"
    )

    await app.state.redis.rpush(
        "JOB_QUEUE",
        json.dumps({
            "type": "COMPLETE_TASK",
            "taskId": str(task_id)
        })
    )

    return Task(**dict(row))

@app.delete("/tasks/{task_id}", response_model=Task)
async def delete_task(task_id: int, user=Depends(get_user_from_auth)):
    async with app.state.db_pool.acquire() as conn:
        row = await conn.fetchrow(
            """
            DELETE from tasks
            WHERE id = $1 AND user_id = $2
            RETURNING id, title, done, status, user_id, created_at
            """,
            task_id, user["user_id"]
        )
        if not row:
            raise HTTPException(status_code=404, detail="Task not found")

    await app.state.redis.delete(
        f"task:{user['user_id']}:{task_id}",
        f"tasks:{user['user_id']}"
    )

    return Task(**dict(row))
