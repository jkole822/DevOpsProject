# Kubernetes Services

## 🌐 What are Services?

- Kubernetes Services enable communication between:
  - Different groups of Pods (frontend ↔ backend ↔ database/external sources)
  - Applications and users (internal + external access)
- They decouple microservices by abstracting pod IPs (which can change).

## 🚪 Why Services?

- Pods get their own internal IP, but:
  - IPs change when Pods are recreated.
  - Internal Pod IPs are not accessible externally.
- Services provide a stable endpoint for communication.

## 🔑 Use Case: External Access

- Pod IP (10.244.0.2) is not directly accessible from outside.
- External user wants to access a web app → solution = Service.
- Service maps:
  - Node’s IP + Port → forwards traffic → Pod’s IP + TargetPort

## 🛠️ Types of Services

1. ClusterIP (default)
   - Creates a virtual IP inside the cluster.
   - Enables internal communication (frontend ↔ backend).
   - Not exposed externally.
2. NodePort
   - Maps a port on the Node to a Pod’s TargetPort.
   - Accessible externally via NodeIP:NodePort.
   - NodePort range: 30000–32767.
3. LoadBalancer
   - Provisions a cloud provider’s load balancer.
   - Distributes traffic across Pods (e.g., multiple frontend replicas).
   - Typically used in cloud environments.

List services:

```bash
kubectl get svc
```

## ⚙️ NodePort Service Deep Dive

- Three ports involved:
  - TargetPort → Port inside Pod (80).
  - Port → Port on the Service object (80).
  - NodePort → Port on the Node (30008).
- Example flow:

```bash
curl http://<NodeIP>:30008 → forwards → Pod’s TargetPort (80)
```

Connect to a Node Port from the host:
- Create tunnel for service:
```bash
minikube service <service-name> 
```

Forward a port on your local machine to a port on a pod inside the cluster:
```bash
kubectl port-forward svc/<service name> <host port>:<target port> 
```

## 📄 Service YAML Definition (NodePort)

```yaml
apiVersion: v1
kind: Service
metadata:
  name: my-service
spec:
  type: NodePort
  selector:
    app: my-app # Matches labels from Pod/Deployment
  ports:
    - port: 80 # Service’s port
      targetPort: 80 # Pod container port
      nodePort: 30008 # External access port (optional)
```

- Selector: Connects Service → matching Pods (via labels).
- If multiple Pods have the same label → Service automatically load balances.

## ⚖️ Load Balancing

- Service automatically distributes requests across multiple Pods.
- Default algorithm = random.
- If Pods are spread across multiple nodes:
  - Service spans all nodes.
  - Same NodePort is available on every Node in the cluster.
  - Access app using any NodeIP:NodePort.

## ✅ Key Takeaways

- Services abstract Pod IPs → provide stable endpoints.
- Three main types: ClusterIP, NodePort, LoadBalancer.
- NodePort enables external access through <NodeIP>:<NodePort>.
- Services automatically load balance across multiple Pods.
- No extra config needed when Pods scale up/down → Service updates automatically.

## Kubernetes Service: ClusterIP Notes

### 🔹 Context

Full-stack web applications have multiple components:

- Frontend web server Pods
- Backend server Pods
- Key-value store Pods (e.g., Redis)
- Database Pods (e.g., MySQL)

These components need to communicate reliably with each other.

### 🔹 Problem

- Pods have dynamic IPs:
  - Pods can go down and be recreated at any time.
  - IP addresses are not static → cannot rely on them for communication.
- Routing decision:
  - Which backend Pod should a frontend Pod connect to?

Kubernetes services solve this problem by acting as a stable interface.

### 🔹 Solution: Kubernetes Service (ClusterIP)

- ClusterIP Service groups a set of Pods together:
  - Provides a single IP and name inside the cluster.
  - Requests to the service are forwarded randomly to one of the Pods under the service.
  - Enables microservices architecture with independent scaling and movement of Pods.
- Examples:
  - Backend Service → groups all backend Pods.
  - Redis Service → allows backend Pods to access Redis Pods via a stable service endpoint.

### 🔹 Creating a ClusterIP Service

1. Definition File Structure:

```yaml
apiVersion: v1
kind: Service
metadata:
  name: backend
spec:
  type: ClusterIP # Default type if not specified
  ports:
    - targetPort: 80 # Port exposed by backend Pods
      port: 80 # Port exposed by the service
  selector:
    app: backend # Labels copied from backend Pods
```

2. Steps

- Create the service using:

```bash
kubectl create -f service-definition.yaml
```

- Verify the service status using:

```bash
kubectl get svc
```

### 🔹 Key Points

- ClusterIP is the default service type.
- Other Pods should use the service name to communicate rather than Pod IPs.
- Services allow decoupled communication, letting Pods move or scale without breaking connections.

## Kubernetes Service: LoadBalancer Type Notes

### 🔹 Context

- Previously discussed NodePort services:
  - Expose applications on a high port on all nodes.
  - External users can access pods via NodeIP:NodePort.
- Example scenario:
  - Frontend apps: Voting App and Result App.
  - Pods may only be deployed on a subset of nodes.
  - NodePort makes them accessible via all nodes in the cluster.

### 🔹 Problem with NodePort

- Users need friendly URLs, e.g., voting.example.com or result.example.com.
- NodePort URLs (IP:Port) are not user-friendly.
- Setting up a traditional external load balancer manually (e.g., Nginx, HAProxy) is tedious:
  - Requires provisioning VMs, installing, configuring, and maintaining the load balancer.

### 🔹 Solution: LoadBalancer Service

- Kubernetes can integrate with cloud provider native load balancers:

  - Supported platforms: GCP, AWS, Azure.
  - Automatically provisions a cloud load balancer for your service.
  - Users can access the service via a single, user-friendly URL.

- How it works:
  - Set the service type to LoadBalancer in the service definition:

```yaml
spec:
  type: LoadBalancer
```

- Kubernetes handles the cloud load balancer creation and routes traffic to NodePort(s) internally.

### 🔹 Notes

- Unsupported environments (e.g., VirtualBox, bare-metal):
  - LoadBalancer type behaves like NodePort.
  - No external load balancer is automatically provisioned.
- Using LoadBalancer simplifies traffic routing and external accessibility on supported cloud platforms.

### 🔹 Summary

- NodePort: exposes service on high port of all nodes, suitable for testing or internal access.
- LoadBalancer: creates a single external endpoint, integrates with cloud provider load balancer, ideal for production-facing applications.
- Always check if your environment supports external load balancers before using this service type.
