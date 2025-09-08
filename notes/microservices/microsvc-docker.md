# Microservices Architecture with Docker

## Overview

- Goal: Understand microservices architecture through a simple web application.
- Deployment: Application will be deployed on multiple Kubernetes platforms (e.g., Google Cloud Platform).
- Demo Application: Sample voting application by Docker.

## Voting Application Architecture

### Components:

1. Voting App
   - Language: Python
   - Function: Provides interface for users to vote between two options (Cat vs Dog)
   - Stores votes in Redis (in-memory database)
2. Worker
   - Language: .NET
   - Function: Processes votes from Redis and updates persistent database (PostgreSQL)
   - Example: If vote = "Cat", increments the "Cat" count in PostgreSQL
3. Results App
   - Language: Node.js
   - Function: Reads vote counts from PostgreSQL and displays results in a web interface

### Data Flow

> User -> Voting App (Python) -> Redis -> Worker (.NET) -> PostgreSQL -> Results App (Node.js) -> User

### Key Points

- Application stack uses different programming languages and services.
- Demonstrates how Docker can orchestrate diverse components.

## Running the Application Stack in Docker

### Assumptions:

- All Docker images are built and available in the Docker repository.

### Step 1: Run Data Layer

- Redis (In-memory database)

```bash
docker run -d --name redis redis
```

- `-d` → Run in background
- `--name redis` → Name container (important for linking later)
- PostgreSQL (Persistent database)

```bash
docker run -d --name db postgres
```

### Step 2: Run Application Services

**Voting Web App**

```bash
docker run -d --name vote -p 5000:80 voting-app
```

- `-p 5000:80` → Map container port 80 to host port 5000

**Results Web App**

```bash
docker run -d --name result -p 5001:82 results-app
```

- `-p 5001:82` → Map container port 82 to host port 5001

**Worker**

```bash
docker run -d --name worker worker-app
```

### Step 3: Linking Containers

- **Problem**:
  - Containers are running but not aware of each other.
  - Example: Voting app cannot find Redis; Worker cannot find PostgreSQL.
- **Solution**: Docker Links
  - Use --link to connect dependent containers.
  - Example for voting app:

```bash
docker run -d --name vote --link redis:redis -p 5000:80 voting-app
```

- `--link redis:redis` → Links container vote to container redis
- Creates an entry in `/etc/hosts` mapping redis to its internal IP

### Linking Results App to Database

```bash
docker run -d --name result --link db:db -p 5001:82 results-app
```

### Linking Worker to Redis and PostgreSQL

```bash
docker run -d --name worker --link redis:redis --link db:db worker-app
```

## Notes:

- Container naming is important for linking.
- `--link` method is deprecated, newer Docker networking methods (e.g., user-defined networks, Docker Swarm) are recommended for production.
- This exercise teaches basic linking and inter-container communication.

## Summary

- Microservices architecture allows using multiple languages and services.
- Docker enables running all services independently yet connected via links or networks.

### Key components:

- Voting App (Python + Redis)
- Worker (.NET + PostgreSQL)
- Results App (Node.js + PostgreSQL)
