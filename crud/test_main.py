from fastapi.testclient import TestClient
from main import app

client = TestClient(app)

def test_health():
    response = client.get("/health")
    assert response.status_code == 200
    assert response.json() == {"status": "ok"}

def create_task(title="Lorem"):
    response = client.post("/tasks", json={"title": title})
    assert response.status_code == 200
    task = response.json()
    assert task["title"] == title
    assert task["done"] is False
    return task

def test_list_tasks():
    task = create_task()
    response = client.get("/tasks")
    assert response.status_code == 200
    tasks = response.json()
    assert any(t["title"] == task["title"] for t in tasks)

def test_update_task():
    task = create_task("Ipsum")
    response = client.put(f"/tasks/{task['id']}", json={"done": True})
    assert response.status_code == 200
    updated_task = response.json()
    assert updated_task["done"] is True

def test_delete_task():
    task = create_task("Dolor")
    response = client.delete(f"/tasks/{task['id']}")
    assert response.status_code == 200
    deleted_task = response.json()
    assert deleted_task["id"] == task["id"]

    response = client.get("/tasks")
    tasks = response.json()
    assert all(t["id"] != task["id"] for t in tasks)

