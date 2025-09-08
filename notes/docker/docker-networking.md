# Docker Networking - Core Concepts

## Default Networks

When Docker is installed, it creates **three default networks**:

1. **bridge** (default for containers)
2. **host**
3. **none**

- Use a specific network type using the `--network=` flag
  - Example `docker run --network=host <container_id>`

---

## Bridge Network

- **Default network** for containers.
- Private, internal network created on the host.
- Containers receive internal IPs (e.g., `172.17.x.x`).
- Containers on the same bridge network can communicate with each other.
- To access a container externally:
  - Map container ports to host ports (`-p host_port:container_port`).
  - Example: `docker run -p 8080:80 nginx`

---

## Host Network

- Removes network isolation between host and container.
- Container uses the host’s network stack directly.
- Example: Running a web server on port `5000` makes it available at `host_ip:5000` automatically, no port mapping required.
- **Limitation**: Cannot run multiple containers on the same host using the same port (since ports are shared with host).

---

## None Network

- Container not attached to any network.
- No access to external networks or other containers.
- Runs in **complete isolation**.

---

## Custom Networks

- By default, Docker provides only one bridge network.
- To isolate containers, you can create custom bridge networks:

```bash
  docker network create --driver bridge --subnet 182.18.0.0/16 --gateway 182.18.0.1 my_network
```

Use `docker run --network <network_name> <container_id>` to start a container on a specific network

Use `docker network ls` to list all networks.

---

## Inspecting Networks

- Use `docker inspect <container_id>` to view:
  - Network type
  - Internal IP address
  - MAC address
  - Other network settings

---

## Container-to-Container

- Communication
  - Containers can communicate using:
    - Internal IP addresses (not reliable, may change after restart).
    - Container names (preferred).

### Docker DNS

- Docker provides a built-in DNS server.
- All containers on a host can resolve each other by name.
- DNS server runs at: 127.0.0.11

---

## Under the Hood: How Networking Works

- Docker uses network namespaces for isolation.
- Virtual Ethernet (veth) pairs connect containers to networks.
  - One end inside the container, one end connected to the bridge/host.

---

## Key Takeaways

- Bridge = default, isolated network with internal IPs, requires port mapping for external access.
- Host = shares host networking, no port mapping needed, but ports may conflict.
- None = no networking, isolated container.
- Containers can communicate using names via Docker’s built-in DNS (127.0.0.11).
- Custom networks allow container grouping and isolation.
- Networking isolation is implemented using namespaces + veth pairs.
