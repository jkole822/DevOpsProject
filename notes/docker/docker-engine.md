# Docker Engine - Core Concepts

## Overview

Docker Engine is the runtime environment for Docker, allowing applications to run in isolated containers. It consists of three main components:

1. **Docker Daemon**
   - A background process that manages Docker objects such as images, containers, volumes, and networks.
2. **Docker REST API**

   - An interface that programs can use to communicate with the Docker daemon and provide instructions.
   - Enables the creation of custom tools interacting with Docker.

3. **Docker CLI (Command Line Interface)**
   - The command line tool used to perform actions like running containers, stopping them, and managing images.
   - Interacts with the Docker daemon through the REST API.
   - Can also manage remote Docker engines using `-H <host:port>` option.

Example:

```bash
docker -H 10.1.23.2:2375 run nginx
```

## Containerization and Isolation

### Namespaces

Docker uses namespaces to isolate resources for containers, making them appear as independent systems. Types of namespaces include:

- Process IDs (PID)
  - Each container has its own PID namespace.
  - Processes inside a container see their own IDs starting from 1, even though the underlying host uses different IDs.
  - Example: Running an Nginx container shows PID 1 inside the container, but a different PID on the host.
- Other namespace types:
  - Network
  - IPC (Inter-Process Communication)
  - Mounts
  - Unix Timesharing Systems

### How it works

- Containers share the host's system resources (CPU, memory).
- Isolation is logical, not physical, thanks to namespaces.

## Resource Management

### Control Groups (Cgroups)

- Docker uses control groups to limit hardware resources for containers.
- Without restrictions, a container may consume all host resources.
  **CPU limitation example:**

```bash
docker run --cpus="0.5" my_container
```

- Limits container to 50% of host CPU.
  **Memory limitation example:**

```bash
docker run --memory="100m" my_container
```

- Limits container memory usage to 100 MB.

## Key Takeaways

- Docker Engine allows running isolated applications using containers.
- Namespaces provide process and resource isolation.
- Cgroups control resource allocation (CPU, memory) to prevent a container from consuming excessive host resources.
- Docker CLI can manage local or remote engines via REST API.
