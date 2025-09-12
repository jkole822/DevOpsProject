# Kubernetes Deployments: Updates & Rollbacks

## Rollouts & Versioning

- **Creating a Deployment** → triggers a **rollout**.
- Each rollout = a new **deployment revision**:
  - First deployment → Revision 1
  - Update (e.g., new container image) → Revision 2
- Enables tracking changes & rolling back if needed.

### Key Commands

- Check rollout status:

```bash
kubectl rollout status deployment/<name>
```

- View rollout history:

```bash
kubectl rollout history deployment/<name>
```

## Deployment Strategies

1. Recreate Strategy

- Destroy all old Pods → then create all new Pods.
- Downtime occurs (app unavailable during switch).
- Not the default.

2. Rolling Update Strategy (default)

- Replaces Pods one at a time.
- Ensures zero downtime.
- Old ReplicaSet scaled down gradually, new ReplicaSet scaled up gradually.

## Updating Deployments

Updates can include:

- New container image (application version upgrade).
- Updated labels.
- Changing replica count.
- Resource allocation changes.

### Methods

1. Modify definition file and apply changes:

```bash
kubectl apply -f deployment.yaml
```

→ triggers new rollout and revision.

2. Update image directly:

```bash
kubectl set image deployment/<name> <container>=<new-image>
```

⚠️ Warning: This may cause drift from your definition file.

## Inspecting Deployment Details

- View detailed events:

```bash
kubectl describe deployment <name>
```

- Recreate strategy: old ReplicaSet scaled to 0, then new one scaled up.
- Rolling update: scale down/up happens incrementally.

## What Happens Under the Hood

- Initial Deployment:
  - Creates a ReplicaSet → which creates Pods.
- Upgrade:
  - New ReplicaSet is created.
  - Old ReplicaSet is scaled down while new one scales up.
  - Visible with:

```bash
kubectl get rs
```

## Rollbacks

- Undo a deployment change:

```bash
kubectl rollout undo deployment/<name>
```

- Effect:
  - New ReplicaSet scaled down.
  - Old ReplicaSet scaled back up.
- Example:
  - Before rollback → old RS = 0 Pods, new RS = 5 Pods.
  - After rollback → old RS = 5 Pods, new RS = 0 Pods.

## Other Notes

- `kubectl run` actually creates a Deployment, not just a Pod:

```bash
kubectl run nginx --image=nginx
```

→ Deployment + ReplicaSet + Pods created automatically.

- Using a definition file is best practice:
  - Can be versioned in source control.
  - Easier to update consistently.

## Summary of Commands

- Create a deployment:

```bash
kubectl create -f deployment.yaml
```

- Create a deployment and record cause of change (`CHANGE-CAUSE` from rollout history):

```bash
kubectl create -f deployment.yaml --record
```

- List deployments:

```bash
kubectl get deployments
```

- Update deployment:

```bash
kubectl apply -f deployment.yaml
kubectl set image deployment/<name> <container name>=<new-image>
```

- Rollout status:

```bash
kubectl rollout status deployment/<name>
```

- Rollout history:

```bash
kubectl rollout history deployment/<name>
```

- Rollback:

```bash
kubectl rollout undo deployment/<name>
```

- Restart:

```bash
kubectl rollout restart deployment/<name>
```

- Rollback to specific revision:

```bash
    kubectl rollout undo deployment/<deployment_name> --to-revision=<revision_number>
```
