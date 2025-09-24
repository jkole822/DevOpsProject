# Ansible Modules Notes

## Overview

- Ansible **modules** are reusable units of code that perform specific tasks.
- Modules are categorized based on functionality: **system, command, file, database, cloud, Windows**, etc.
- Documentation and comprehensive module lists: [docs.ansible.com](https://docs.ansible.com)
- Use `ansible-doc -l` to list available modules.

## Module Categories

### 1. System Modules

- Perform system-level operations:
  - Manage users and groups
  - Configure IP tables/firewalls
  - Work with logical volumes
  - Mounting operations
  - Manage services (start, stop, restart)

### 2. Command Modules

- Execute commands or scripts on a host.
- Examples:
  - `command` → runs simple commands
  - `shell` → executes shell commands
  - `script` → runs local scripts on remote hosts
- Supports **free-form commands** (key-value format optional).

### 3. File Modules

- Work with files and directories:
  - `acl` → manage ACLs
  - `archive` → compress/uncompress files
  - `lineinfile` → modify lines in files
  - `replace` → replace file content

### 4. Database Modules

- Work with databases such as **MongoDB, MySQL, PostgreSQL**:
  - Add/remove databases
  - Modify database configurations

### 5. Cloud Modules

- Manage cloud providers like **AWS, Azure, Docker, Google Cloud, OpenStack, VMware**.
- Examples of tasks:
  - Create/destroy instances
  - Configure networking/security
  - Manage containers, data centers, clusters

### 6. Windows Modules

- Perform tasks in Windows environment:
  - `win_copy` → copy files
  - `win_command` → run commands
  - `win_iis_website` → manage IIS websites
  - `win_msi` → install software via MSI
  - `win_reg` → manage registry
  - Manage services and users

## Key Ansible Modules Explained

### Command Module

- Executes a command on a remote host.
- **Syntax example:**

```yaml
- name: Print date
  command: date

- name: List file contents
  command: cat /etc/resolv.conf
```

- **Parameters**:
  - `chdir` → change directory before executing command
  - `creates` → run command only if a file/folder does not exist
  - `free-form` → allows running commands without strict key-value format
- **Script Module**
- Executes a local script on one or more remote hosts.
- Ansible automatically copies the script to the target hosts.
- Example:

```yaml
- name: Run local script on remote host
  script: /path/to/local/script.sh
```

- **Service Module**
  - Manages system services: **start, stop, restart, enable, disable**.
  - Example:

```yaml
- name: Start PostgreSQL service
  service:
    name: postgresql
    state: started

- name: Start HTTPD service
  service:
    name: httpd
    state: started
```

- Key concept: Idempotency
  - `started`, `stopped`, `restarted` ensure the service reaches the desired state.
  - Running the playbook multiple times does not duplicate actions.

**Lineinfile Module**

- Ensures specific lines exist in a file; adds or replaces if necessary.
- Useful for configuration files.
- Example:

```yaml
- name: Add DNS server
  lineinfile:
    path: /etc/resolv.conf
    line: "nameserver 8.8.8.8"
```

- Idempotent: Running multiple times does not duplicate entries.

## Free-form vs Parameterized Input

- Command module supports free-form input:

```yaml
command: cat /etc/resolv.conf
```

- Copy or service module requires parameterized input:

```yaml
copy:
  src: /tmp/file.txt
  dest: /home/user/file.txt
```

## Summary

- Modules abstract tasks in a reusable way.
- Categories: system, command, file, database, cloud, Windows.
- Key modules covered:
  - `command` → run commands
  - `script` → run local scripts remotely
  - `service` → manage services (idempotent)
  - `lineinfile` → ensure file lines (idempotent)
- Idempotency ensures safe repeated runs.
- Modules use YAML syntax: either free-form or key-value parameters.
