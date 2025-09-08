# Container Orchestration Notes

## Why Container Orchestration?

- Packaging applications into Docker containers is only the first step.
- Challenges in production:
  - Apps often rely on other containers (databases, messaging services, etc.).
  - Need to handle **scaling up** (more users/load) and **scaling down** (less load).
- Requirements:
  - Platform to orchestrate container connectivity.
  - Automatic scaling (up/down).
  - Automated deployment and management.

---

## What is Container Orchestration?

- **Definition**: The process of automatically deploying and managing containers.
- Ensures:
  - Connectivity between containers.
  - Scaling based on load.
  - High availability.

---

## Popular Container Orchestration Tools

- **Docker Swarm**
  - Easy to set up and start.
  - Lacks advanced features for complex applications.
- **Apache Mesos**
  - Supports advanced features.
  - Difficult to set up and get started.
- **Kubernetes (from Google)**
  - Most popular option.
  - Difficult to set up initially.
  - Highly customizable.
  - Supports deployment of **complex architectures**.
  - Widely supported on cloud providers: **AWS, Azure, GCP**.
  - Top-ranked open source project on GitHub.

---

## Advantages of Container Orchestration

- **High availability**
  - Hardware failures don’t bring the app down → multiple instances across nodes.
- **Load balancing**
  - User traffic distributed across containers.
- **Scalability**
  - Scale up when demand increases.
  - Scale down when demand decreases.
- **Resource flexibility**
  - Scale underlying nodes (infrastructure) up or down without downtime.
- **Declarative management**
  - Use configuration files (YAML/JSON) to describe desired state.
  - Platform ensures actual state matches desired state.

---

## Kubernetes

- Industry-standard container orchestration tool.
- Manages **hundreds or thousands of containers** in a clustered environment.
- Provides:
  - Automated deployment
  - Scaling
  - Load balancing
  - Fault tolerance
  - Declarative configuration

---

## What is a Node?

A node is simply a machine (physical server or virtual machine) that is part of the container orchestration cluster.

Each node provides compute resources (CPU, RAM, storage, networking) that can run containers.

Types of Nodes in Kubernetes (for example)

- Master Node (Control Plane)
  - Responsible for managing the whole cluster.
  - Decides where containers (pods) run, monitors health, handles scaling, etc.
  - Runs Kubernetes components like kube-apiserver, scheduler, and controller-manager.
- Worker Node
  - Where your actual containers (pods) run.
  - Each worker node runs:
    - Kubelet (agent that talks to master and ensures containers are running).
    - Container runtime (Docker, containerd, etc.) to start/stop containers.
    - Kube-proxy for networking between services.

### Analogy

Think of a node like a computer in a big warehouse of computers:

- Cluster = whole warehouse.
- Nodes = individual computers in the warehouse.
- Containers = apps running inside those computers.
- The orchestrator (Kubernetes) decides:
  - Which computer should run which app.
  - How many copies of an app to run.
  - What happens if one computer fails.

### Example

Suppose you have a cluster with 3 nodes:

- Node A: running 5 web containers.
- Node B: running 3 database containers.
- Node C: idle, waiting for more load.

When traffic increases, Kubernetes may schedule new containers on Node C to spread the load.
