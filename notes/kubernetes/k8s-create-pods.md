# Create Pods with YAML

## Purpose of YAML in Kubernetes

- Kubernetes uses **YAML files** as input for creating objects such as:
  - Pods
  - ReplicaSets
  - Deployments
  - Services
- All Kubernetes YAML definitions follow a **similar structure**.

---

## Four Required Top-Level Fields

Every Kubernetes YAML file must include these **root-level fields**:

1. **apiVersion**

   - Specifies the version of the Kubernetes API being used.
   - Must match the object type.
   - Examples:
     - `v1` (for core objects like Pods)
     - `apps/v1`
     - `extensions/v1beta1`

2. **kind**

   - Defines the **type of object**.
   - Examples:
     - `Pod`
     - `ReplicaSet`
     - `Deployment`
     - `Service`

3. **metadata**

   - Provides information about the object.
   - Structure: **dictionary** (key–value pairs).
   - Common properties:
     - `name`: string (e.g., `my-app-pod`)
     - `labels`: dictionary (e.g., `app: myapp`)
   - **Indentation rules**:
     - Properties under `metadata` must be indented **more than metadata**.
     - Sibling properties (e.g., `name` and `labels`) must have the same indentation level.
   - Purpose of labels:
     - Helps organize and filter objects later.
     - Example: label pods as `frontend`, `backend`, or `database`.

4. **spec**
   - Specifies the **desired state** of the object.
   - Different for each object type.
   - For Pods:
     - `containers`: list of containers in the Pod.
     - Each container entry is a **dictionary** with fields like:
       - `name`: container name.
       - `image`: container image (e.g., `nginx`).
   - Example:
     ```yaml
     spec:
       containers:
         - name: nginx-container
           image: nginx
     ```

---

## Workflow Summary

1. Define the **four top-level fields**: `apiVersion`, `kind`, `metadata`, `spec`.
2. Add details specific to the object (e.g., container image for a Pod).
3. Create the object:

```bash
kubectl create -f pod-definition.yaml
```

4. Verify:

- List Pods:

```bash
kubectl get pods
```

- Inspect details of a Pod:

```bash
kubectl describe pod <pod-name>
```

## Key Takeaways

- YAML indentation matters – sibling properties must align correctly.
- Metadata is restricted to what Kubernetes expects (e.g., name, labels).
- Labels are flexible – you can use any key-value pairs to group and filter objects.
- Spec changes depending on the object type – always refer to Kubernetes documentation for details.
