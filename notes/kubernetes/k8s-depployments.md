# Kubernetes Deployments

## Why Deployments?

Deployments solve common needs in **production environments**:

- Run **multiple instances** of your application (for availability & scaling).
- **Seamlessly upgrade** to newer versions of container images.
- Perform **rolling updates** (upgrade Pods one at a time, not all at once).
- **Rollback** to a previous version if an upgrade fails.
- **Pause and resume** changes to apply multiple updates together (e.g., scaling + resource changes).

---

## Relationship in Kubernetes Hierarchy

- **Pod** → Single instance of an application.
- **ReplicaSet** → Ensures multiple Pods are running.
- **Deployment** → Manages ReplicaSets and provides:
  - Rolling updates
  - Rollbacks
  - Pause/resume of changes

**Hierarchy**:  
`Deployment → ReplicaSet → Pods`

---

## Deployment Definition File

- Very similar to a ReplicaSet YAML.
- Difference: `kind: Deployment`

### Structure

- **apiVersion**: `apps/v1`
- **kind**: `Deployment`
- **metadata**: Name & labels
- **spec**:
  - `replicas`: Desired number of Pods
  - `selector`: Matches Pods
  - `template`: Pod definition (containers, images, etc.)

---

## Commands

- Create a Deployment:

```bash
kubectl create -f deployment.yaml
```

- List Deployments:

```bash
kubectl get deployments
```

- List ReplicaSets:

```bash
kubectl get rs
```

- List Pods:

```bash
kubectl get pods
```

- List all objects:

```bash
kubectl get all
```

## Key Takeaway

- At first, Deployments look similar to ReplicaSets.
- The real advantages of Deployments appear in:
  - Rolling updates
  - Rollbacks
  - Pausing & resuming rollouts
