# Kubernetes Networking Basics

## Single-Node Networking

- Each **node** has an IP (e.g., `192.168.1.2`).
  - Used for SSH or direct access to the node.
  - In Minikube → this is the VM’s IP inside the hypervisor, not the laptop’s IP.
- **Pods get their own internal IPs**, not containers.
  - Example: Pod IP `10.244.0.2` from network `10.244.0.0/16`.
- Pods communicate with each other using these internal IPs.
- ⚠️ Problem: Pod IPs are **ephemeral** (change if Pods are recreated).  
  → Using raw Pod IPs for communication is unreliable.

---

## Multi-Node Networking

- Nodes each have their own IPs (e.g., `192.168.1.2`, `192.168.1.3`).
- Each node initially assigns Pods from the same internal subnet (e.g., `10.244.0.0/16`).
  - Leads to **IP conflicts** when nodes join a cluster (Pods on different nodes may get the same IP).
- Kubernetes by itself does **not** set up networking between nodes and Pods.

---

## Kubernetes Networking Requirements

Kubernetes expects networking to meet these principles:

1. **All Pods can communicate** with each other without NAT.
2. **All Nodes can communicate** with Pods.
3. **All Pods can communicate** with Nodes.

---

## Networking Solutions (CNI Plugins)

Instead of building networking manually, use **pre-built solutions**:

- **Flannel**
- **Calico**
- **Weave Net** (used in Play with Kubernetes labs)
- **Cilium**
- **Cisco ACI**
- **VMware NSX-T**
- **Big Cloud Fabric**

Choice depends on platform:

- From scratch → Flannel, Calico, Weave Net.
- VMware → NSX-T.
- Cloud providers often integrate their own solutions.

---

## How CNI Plugins Work

- Assign **different subnets** to each node.
- Ensure **unique Pod IPs** across the cluster.
- Use **routing techniques** to allow:
  - Pod ↔ Pod communication (across nodes).
  - Pod ↔ Node communication.
  - Node ↔ Pod communication.

---

## Key Takeaway

- Kubernetes delegates networking setup to CNI plugins.
- These plugins create a **flat, routable network**:
  - Every Pod gets a unique IP.
  - Pods and Nodes can communicate directly.

---

## Definitions

### Flat network

- Every Pod in the cluster gets its own unique IP address.
- These IPs are all in a single address space (like 10.244.0.0/16).
- No NAT or address translation is needed for Pod-to-Pod communication.

👉 From Pod A’s point of view, Pod B is “just another IP” on the same network.

### Routable network

- The network is set up so that any Pod can reach any other Pod directly, even across nodes.
- The routing rules are configured cluster-wide (by the CNI plugin, e.g., Flannel, Calico, Cilium).
- If Pod A is on Node 1 and Pod B is on Node 2, the cluster routes the traffic automatically.

### Network Address Translation (NAT)

It’s a technique where a network device (like a router) modifies the source or destination IP address in a packet as it passes through.

### Why NAT is used

- IP conservation: Inside a home/office, many devices can share one public IP.
- Example: Your laptop might be 192.168.0.5, but when you access Google, the router translates it to the public IP (like 73.21.45.10).
- Security: Hides internal IPs from the outside world.
- Flexibility: Lets private networks communicate with public networks.

### Types of NAT

- SNAT (Source NAT)
  - Changes the source IP of packets leaving a private network.
  - E.g., 192.168.0.5 → 73.21.45.10.
- DNAT (Destination NAT)
  - Changes the destination IP of packets coming in.
  - Used for port forwarding (e.g., forward requests from 73.21.45.10:8080 to 192.168.0.5:80).
- PAT (Port Address Translation)
  - A form of NAT where many devices share a single public IP, but are distinguished by port numbers.
  - This is what home routers usually do.

### Why Kubernetes cares about NAT

- By default, Kubernetes does not want Pods to require NAT to talk to each other.
- Instead, it relies on CNI plugins to build a flat network where:
  - Pod IPs are unique across the cluster.
  - Pods can reach each other directly (no address translation).
- This makes service discovery and communication simpler and faster.
