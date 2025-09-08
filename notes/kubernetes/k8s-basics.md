# Kubernetes Basics: Nodes, Clusters, and Components

## Nodes

- A **node** = a machine (physical or virtual) with Kubernetes installed.
- Nodes are **worker machines** → where containers are launched.
- Formerly called **Minions** (term still used sometimes).
- If a node fails → applications running on it go down.

---

## Clusters

- A **cluster** = a group of nodes.
- Benefits:
  - **High availability**: if one node fails, workloads still run on others.
  - **Load sharing**: multiple nodes distribute workloads.

---

## Master Node

- The **master** manages the cluster.
- Responsibilities:
  - Tracks cluster membership (nodes).
  - Monitors nodes.
  - Reschedules workloads from failed nodes.
  - Orchestrates containers across worker nodes.

---

## Core Kubernetes Components

When you install Kubernetes, you install several key components:

1. **API Server**

   - Frontend for Kubernetes.
   - All users, tools, and CLIs (e.g., `kubectl`) communicate through the API server.

2. **etcd**

   - A distributed, reliable **key-value store**.
   - Stores all cluster data (nodes, workloads, configs).
   - Provides distributed consistency and locks to avoid conflicts between masters.

3. **Scheduler**

   - Assigns newly created containers (pods) to available worker nodes.

4. **Controllers**

   - The “brain” of orchestration.
   - Watch cluster state (nodes, containers, endpoints).
   - Take action if something fails (e.g., restart a container).

5. **Container Runtime**

   - The software that runs containers.
   - Examples: **Docker** (common), **rkt**, **CRI-O**.
   - Kubernetes is runtime-agnostic.

6. **Kubelet**
   - Agent running on **each worker node**.
   - Ensures containers are running as expected.
   - Reports node health back to the master.
   - Executes master’s instructions.

---

## Role of Master vs Worker

- **Master node**:
  - Runs: API Server, etcd, Controller Manager, Scheduler.
- **Worker node (Minion)**:
  - Runs: Kubelet, Container Runtime (Docker), and hosts containers.

---

## Kubectl (Kube Control)

- Command-line utility to interact with Kubernetes.
- Common commands:
  - `kubectl run <app>` → Deploy an application.
  - `kubectl cluster-info` → View cluster information.
  - `kubectl get nodes` → List all nodes in the cluster.
- Used for: deploying, managing apps, viewing cluster state, troubleshooting.

---

## Summary

- **Node** = single worker machine.
- **Cluster** = group of nodes (provides HA + load sharing).
- **Master node** manages orchestration with API server, etcd, scheduler, and controllers.
- **Worker nodes** host containers and run kubelet + runtime.
- **Kubectl** = primary CLI tool to interact with the cluster.
