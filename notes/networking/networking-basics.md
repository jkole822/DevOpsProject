# Networking Basics – Lecture Notes

## What is a Network?

- A network connects two or more systems (laptops, desktops, VMs, etc.) for communication.
- Systems connect through a switch, which enables communication within the same network.
- Each system requires a network interface (physical or virtual).
- View interfaces:

```bash
ip link
```

## Assigning IP Addresses

- Example network: 192.168.1.0/24
- Assign IP addresses using:

```bash
ip addr add <IP_ADDRESS>/<PREFIX> dev <INTERFACE>
```

- Once interfaces are up and IPs assigned, hosts can communicate within the same network.

## Switches

- A switch only allows communication within a network.
- Example:
  - Network 1: Systems A & B → 192.168.1.0/24
  - Network 2: Systems C & D → 192.168.2.0/24
- Switch cannot connect systems across different networks.

## Routers

- A router connects multiple networks.
- Assigned one IP per connected network:
  - Network 1 IP: 192.168.1.1
  - Network 2 IP: 192.168.2.1
- Enables communication between 192.168.1.0 and 192.168.2.0.

## Gateways

- Systems must know the gateway (the "door" to other networks).
- View routes:

```bash
routes
```

- Add a route:

```bash
ip route add <DEST_NETWORK>/<PREFIX> via <GATEWAY_IP>
```

### Example

- System B (192.168.1.11) wants to reach System C (192.168.2.10):
- Add route on System B:

```bash
ip route add 192.168.2.0/24 via 192.168.1.1
```

- Add reciprocal route on System C:

```bash
ip route add 192.168.1.0/24 via 192.168.2.1
```

### Default Gateway

- Instead of adding routes for every external network:
  - Use a default route for all unknown destinations:

```bash
ip route add default via <ROUTER_IP>
```

- Equivalent to:

```bash
ip route add 0.0.0.0/0 via <ROUTER_IP>
```

- Ensures all traffic not matching a known network goes through the router.

### Multiple Routers

- You may have separate routers for:
  - Internet
  - Internal private network
- Each needs its own routing entry.

### Troubleshooting Internet Access

- If systems cannot reach the internet:
  - Check the routing table
  - Ensure the default gateway is configured

## Linux Host as a Router

### Example Setup

- Host A: 192.168.1.5
- Host B (router): 192.168.1.6 and 192.168.2.6
- Host C: 192.168.2.5

### Problem

- A cannot reach C (ping fails: network unreachable).
- Solution:
  - On Host A:

```bash
ip route add 192.168.2.0/24 via 192.168.1.6
```

- On Host C:

```bash
ip route add 192.168.1.0/24 via 192.168.2.6
```

### Enabling Packet Forwarding

- By default, Linux does not forward packets between interfaces (for security).
- Check/enable IP forwarding:

```bash
cat /proc/sys/net/ipv4/ip_forward
# 0 = disabled, 1 = enabled

# Enable temporarily
echo 1 > /proc/sys/net/ipv4/ip_forward
```

- To persist across reboots:
  - Edit /etc/sysctl.conf and set:

```bash
net.ipv4.ip_forward = 1
```

## Key Commands Summary

- List interfaces

```bash
ip link
```

- View IP addresses

```bash
ip addr
```

- Assign IP address

```bash
ip addr add <IP>/<PREFIX> dev <INTERFACE>
```

- View routing table

```bash
ip route
```

- Add route

```bash
ip route add <DEST>/<PREFIX> via <GATEWAY>
```

- Default gateway

```bash
ip route add default via <ROUTER_IP>
```

- Enable IP forwarding

```bash
echo 1 > /proc/sys/net/ipv4/ip_forward
```
