# Kubernetes Pods

## Prerequisites

- Application is already built into Docker images and stored in a repository (e.g., Docker Hub).
- A Kubernetes cluster is already running (single-node or multi-node).
- All cluster services must be in a running state.

## What is a Pod?

- The smallest deployable unit in Kubernetes.
- Encapsulates one or more containers.
- Represents a single instance of an application.
- Kubernetes never deploys containers directly onto worker nodes → containers always run inside pods.

## Pods and Scaling

- 1:1 relationship (usually) between a pod and a container.
- To scale up: create additional pods.
- To scale down: delete pods.
- You do not add more containers to an existing pod to scale.

### Example:

- Start: 1 pod → 1 container → 1 app instance.
- More users → create more pods, each with a new container instance.
- If node runs out of capacity → deploy pods on additional nodes in the cluster.

## Multi-Container Pods

- While rare, a pod can contain multiple containers.
- Typical use case: helper containers that support the main app.
  - e.g., a sidecar container that processes files or fetches data.
- Properties:
  - Containers in a pod share:
    - Network namespace (communicate via localhost).
    - Storage volumes.
    - Lifecycle (created/destroyed together).

## Docker Analogy

Without Kubernetes:

- Deploy app with docker run python-app.
- Scale by running multiple docker run commands.
- Adding a helper container requires:
  - Manual networking setup (links, custom networks).
  - Manual shared storage configuration.
  - Manual mapping of app ↔ helper containers.
  - Manual cleanup if app container dies.

With Kubernetes Pods:

- Define containers in the pod spec.
- Kubernetes automatically manages:
  - Networking
  - Shared storage
  - Lifecycle dependency (if app dies → helper dies too)

## Deploying a Pod

- `kubectl run` creates a pod (not just a container).
- Example:

```bash
kubectl run nginx --image=nginx
```

- Downloads image from Docker Hub (or private repo).
- Creates a pod running an NGINX container.

## Viewing Pods

- List pods:

```bash
kubectl get pods
```

- Pod lifecycle:
  - ContainerCreating → pulling image & starting.
  - Running → container is active.

## Accessing Pods

- By default, pods are not exposed externally.
- Can only be accessed internally (from the node).
- Later (when learning about Services & Networking) → expose pods to external users.

✅ Key Takeaways:

- Pods = smallest Kubernetes unit.
- Usually 1 container per pod (scaling = more pods).
- Multi-container pods = special use cases (helper/sidecar pattern).
- `kubectl run` → creates pods, not just containers.
- Pods share network, storage, and lifecycle.
