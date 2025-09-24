# Ansible Inventory Configuration – Notes

## 1. Overview

- **Ansible** can work with one or multiple systems at the same time.
- It establishes **connectivity** to target systems to perform automation tasks.
- Connectivity methods:
  - **Linux:** SSH
  - **Windows:** PowerShell Remoting

## 2. Agentless Architecture

- **Agentless** means no additional software/agent needs to be installed on target machines.
- Ansible only requires:
  - SSH connectivity for Linux
  - WinRM/PowerShell connectivity for Windows
- This is a key difference from many other orchestration tools, which require installing an agent.

## 3. Inventory File

- **Purpose:** Stores information about target systems.
- **Default location:** `/etc/ansible/hosts`
- **Format:** INI-like format

### Example: Simple Inventory

```ini
server1.company.com
server2.company.com
server3.company.com
```

### Grouping Hosts

```ini
[webservers]
web1.company.com
web2.company.com

[dbservers]
db1.company.com
db2.company.com
```

- Use square brackets [ ] to define a group name.
- Multiple groups can exist in the same file.
- Can group groups under a parent group:

```ini
[<parent_name>:children]
child_group1
child_group2
```

## 4. Using Aliases

- You can assign aliases to servers with inventory parameters.

Example:

```ini
web1 ansible_host=192.168.1.10
db1 ansible_host=192.168.1.20
```

- `ansible_host` specifies the **FQDN or IP address** of the server.

## 5. Common Inventory Parameters

| Parameter                    | Purpose                                                           |
| ---------------------------- | ----------------------------------------------------------------- |
| ansible_host                 | Target server IP or hostname                                      |
| ansible_user                 | User for remote connection (default: `root` on Linux)             |
| ansible_port                 | SSH port (default is `22`)                                        |
| ansible_ssh_private_key_file | Path to SSH private key file                                      |
| ansible_connection           | Connection type (`ssh`, `winrm`, `local`)                         |
| ansible_ssh_pass             | SSH password for Linux hosts (not recommended for production)     |
| ansible_password             | WINRM password for Windows hosts (not recommended for production) |

### Example with Parameters

```ini
web1 ansible_host=192.168.1.10 ansible_user=ubuntu ansible_port=2222 ansible_connection=ssh
```

## 6. Localhost Setup

- If you don’t have multiple servers, you can use `localhost`:

```ini
[local]
localhost ansible_connection=local
```

## 7. Security Note

- Storing passwords (`ansible_ssh_pass`) in plain text is not secure.
- Best practice: Use SSH key-based, passwordless authentication for production environments.
- For learning/demo setups, username + password is acceptable to keep things simple.
