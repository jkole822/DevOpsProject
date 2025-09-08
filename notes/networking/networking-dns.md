# DNS Basics in Linux – Lecture Notes

## What is DNS?

- DNS (Domain Name System): Resolves human-readable names (e.g., db, google.com) into IP addresses.
- Without DNS, you would need to use raw IPs to communicate between systems.

## `/etc/hosts` – Local Name Resolution

- Example: Two systems
  - System A: 192.168.1.10
  - System B: 192.168.1.11 (database server)
- Instead of remembering 192.168.1.11, you can map it to db.

### Steps

1. On System A, edit /etc/hosts and add:

```bash
192.168.1.11   db
```

2. Now you can ping db instead of the IP.

3. Notes:

- Host A trusts the `/etc/hosts` entry—it doesn’t verify it.
- Example: You can map 192.168.1.11 to google.com, and Host A will believe it.
- Multiple aliases can be added in /etc/hosts.

### Limitations

- Works well for small networks.
- Not scalable (IP changes → must update every host).

## Centralized DNS Server

- To avoid managing multiple /etc/hosts files, use a DNS server.
- Hosts are configured to query the DNS server for resolution.
- Example: DNS server IP = 192.168.1.100

### Configure on each host:

- File: `/etc/resolv.conf`
- Add:

```bash
nameserver 192.168.1.100
```

- Now, if a hostname is unknown locally, it queries the DNS server.
- If a host IP changes → update DNS server once, all clients resolve correctly.

### Combining Local and DNS Resolution

- Hosts can use both `/etc/hosts` and DNS.
- **Order of lookup** defined in `/etc/nsswitch.conf`, line:

```bash
hosts: files dns
```

- First check `/etc/hosts`
- Then check DNS server

- Order can be modified.

## Public DNS & External Lookups

- If hostname not found locally or in internal DNS → query public DNS.
- Example: Add Google’s public DNS in `/etc/resolv.conf`:

```bash
nameserver 8.8.8.8
```

- Internal DNS servers can also be configured to forward unknown requests to public DNS.

## Domain Names and Structure

- Example: www.google.com
  - . → Root
  - .com → Top-Level Domain (TLD)
  - google → Domain name
  - www → Subdomain
- Subdomains group services:
  - maps.google.com, drive.google.com, mail.google.com

### Organizational Example

- Company domain: mycompany.com
- Subdomains:
  - www.mycompany.com → website
  - mail.mycompany.com → email
  - hr.mycompany.com → HR app

## Search Domains

- Simplify hostname use inside organization.
- Add in `/etc/resolv.conf`:

```bash
search mycompany.com
```

- Now typing ping web → system tries web.mycompany.com.
- Multiple search domains can be specified:

```bash
search mycompany.com product.mycompany.com
```

## DNS Record Types

- A Record → IPv4 → Hostname to IP
- AAAA Record → IPv6 → Hostname to IP
- CNAME Record → Alias (hostname to another hostname)

## Tools for DNS Testing

- `ping` → quick test, but not always reliable for DNS validation.
- `nslookup` → queries DNS server directly (ignores /etc/hosts).
- `dig` → detailed query results from DNS server.

## Key Takeaways

- `/etc/hosts`: local hostname → IP mappings (useful but not scalable).
- `/etc/resolv.conf`: configure DNS servers and search domains.
- `/etc/nsswitch.conf`: defines lookup order (`files` → `/etc/hosts`, `dns` → DNS server).
- DNS hierarchy: Root → TLD → Domain → Subdomain.
- Record types: A, AAAA, CNAME (others exist).
- Tools: `ping`, `nslookup`, `dig` for troubleshooting.
