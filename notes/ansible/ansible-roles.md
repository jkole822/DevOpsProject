# Ansible Roles

## Introduction

- **Concept:** Similar to assigning roles to people (doctor, engineer, chef), in Ansible we assign **roles to servers** to define their purpose.
- Examples:

  - Database server → MySQL
  - Web server → Nginx
  - Messaging server → Redis
  - Backup server

- Assigning a role in Ansible involves all tasks required to make a server functional for that role:
  - Installing prerequisites
  - Installing software packages
  - Configuring services
  - Applying custom configurations (databases, web pages, etc.)

## Why Use Roles?

- **Reusability:**
  - Tasks for a role (e.g., installing MySQL) can be shared and reused across many projects.
- **Code Organization:**
  - Roles enforce best practices for structuring Ansible code.
- **Sharing:**
  - Roles can be shared with others via Ansible Galaxy, a community platform for roles.

## Role Directory Structure

A typical role has the following structure:
my_role/
├── tasks/ # Main task files
├── handlers/ # Handlers for notifications
├── vars/ # Variables used by tasks
├── defaults/ # Default variable values
├── templates/ # Jinja2 templates
├── files/ # Static files to copy
├── meta/ # Role metadata

## Creating a Role

- Use `ansible-galaxy init` to generate a role skeleton:

```bash
ansible-galaxy init my_role
```

- Move existing playbook code into the appropriate directories:
  - Tasks → `tasks/main.yml`
  - Handlers → `handlers/main.yml`
  - Variables → `vars/main.yml` or `defaults/main.yml`
  - Templates → `templates/`

## Using a Role in a Playbook

- Basic example:

```yaml
- hosts: db_servers
  roles:
    - my_mysql_role
```

- Role lookup:
  - Place the role in a `roles` directory inside your playbook folder.
  - Or use the system-wide default location: `/etc/ansible/roles`.
  - Configurable via `roles_path` in `ansible.cfg`.

## Sharing Roles

- Roles can be shared via Ansible Galaxy:
  1. Upload role to a GitHub repository.
  2. Install via `ansible-galaxy install <role_name>`.
  3. Roles are extracted to the default roles directory.

## Finding Roles on Ansible Galaxy

- Search roles via:
  - Galaxy UI
  - CLI:

```bash
ansible-galaxy search <role_name>
```

- Install roles:

```bash
ansible-galaxy install <role_name>
```

- Optionally install in the current directory:

```bash
ansible-galaxy install -p ./roles <role_name>
```

## Advanced Role Usage

- Roles can be specified as arrays of strings:

```yaml
roles:
  - my_mysql_role
  - my_web_role
```

- Or as arrays of dictionaries to pass additional options:

```yaml
roles:
  - role: my_mysql_role
    become: yes
    vars:
    mysql_user_name: admin
```

## Summary

- Roles make it easier to develop, reuse, and share playbooks.
- Help organize tasks, handlers, variables, and templates.
- Can assign multiple roles to a server or multiple servers.
- Use `ansible-galaxy` commands to create, search, install, and manage roles.
- Check default role paths and configurations using:

```bash
ansible-galaxy list
ansible-config dump | grep ROLE
```
