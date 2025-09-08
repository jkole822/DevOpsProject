# Kubernetes Controllers – Key Concepts

## Introduction

- **Controllers** are the brain behind Kubernetes.
  - They monitor Kubernetes objects and respond accordingly.
- **Replication Controller** is one type of controller focused on Pods.

---

## Replication Controller

### Purpose

- Ensures **high availability** by running multiple instances of a Pod.
- Automatically creates a new Pod if an existing one fails.
- Helps **balance load** across multiple Pods and nodes.
- Supports scaling from **1 to N Pods**.

### Differences Between Replication Controller and Replica Set

| Feature     | Replication Controller | Replica Set              |
| ----------- | ---------------------- | ------------------------ |
| Technology  | Older                  | Newer, recommended       |
| Selector    | Optional               | Required (`matchLabels`) |
| API Version | `v1`                   | `apps/v1`                |

---

## Replication Controller Definition File

### Top-Level Fields

1. `apiVersion`: `v1`
2. `kind`: `ReplicationController`
3. `metadata`:
   - `name`: e.g., `my-app-rc`
   - `labels`: key-value pairs (e.g., `app: myapp`, `type: frontend`)
4. `spec`:
   - `replicas`: number of Pods to maintain
   - `template`: Pod definition (nested Pod metadata and spec)

### Key Notes

- `template` section contains **Pod metadata and spec**.
- Two `metadata` and `spec` sections exist: one for RC, one for Pod.
- `replicas` and `template` are siblings and must be aligned.

### Commands

- Create RC:

```bash
kubectl create -f rc-definition.yaml
```

- List RCs:

```bash
kubectl get replicationcontrollers
```

- List Pods created by RC:

```bash
kubectl get pods
```

## Replica Set

### Overview

- Similar to Replication Controller.
- Uses API version: apps/v1.
- Requires a selector to identify Pods it manages.
  - `matchLabels` matches Pod labels.
- Template section still required to create new Pods if any fail.
- Can monitor existing Pods created outside of the Replica Set.

### Replica Set Definition

- Top-level fields: apiVersion, kind, metadata, spec
- `spec` includes:
  - `replicas`: number of Pods
  - `template`: Pod definition
  - `selector`: required field, defines which Pods to monitor

### Example

```yaml
apiVersion: apps/v1
kind: ReplicaSet
metadata:
  name: my-app-rs
spec:
  replicas: 3
  selector:
    matchLabels:
      app: my-app
  template:
    metadata:
      labels:
        app: my-app
    spec:
      containers:
        - name: nginx
          image: nginx:latest
```

### Scaling Replica Sets

1. Update replicas in YAML file and run:

```bash
kubectl replace -f rs-definition.yaml
```

2. Or use kubectl scale:

```bash
kubectl scale --replicas=6 -f rs-definition.yaml
# or
kubectl scale --replicas=6 replicaset my-app-rs
```

- Note: Scaling via command does not update the YAML file automatically.

## Labels and Selectors

- Labels: Key-value pairs assigned to Pods for identification.
- Selectors: Filters used by controllers to monitor specific Pods.
  - Replica Set uses matchLabels to determine which Pods belong to it.
- Labels and selectors are widely used throughout Kubernetes for filtering and organizing resources.

## Commands Summary

| Command                               | Purpose                                |
| ------------------------------------- | -------------------------------------- |
| `kubectl create -f <file>`            | Create any Kubernetes object from YAML |
| `kubectl get replicaset`              | List Replica Sets                      |
| `kubectl get pods`                    | List Pods created by a controller      |
| `kubectl delete replicaset <name>`    | Delete a Replica Set                   |
| `kubectl replace -f <file>`           | Update or replace an existing object   |
| `kubectl scale --replicas=N <object>` | Scale object to N replicas             |

## Key Takeaways

- Replication Controller ensures a specified number of Pods are always running.
- Replica Set is the newer, recommended approach with a required selector.
- Pod templates are necessary even if Pods already exist to maintain desired state.
- Labels and selectors are critical for monitoring, filtering, and organizing resources.
- Scaling can be done declaratively (YAML) or imperatively (command line).
