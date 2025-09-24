# Introduction to Ansible

## What is Ansible?

- **Ansible** is a powerful IT automation tool.
- Designed to be:
  - **Simple enough** for anyone in IT to learn quickly.
  - **Powerful enough** to automate even the most complex deployments.
- Reduces the need for developing and maintaining complex scripts.
- Tasks that once required extensive scripting can now be done with just a few lines in an **Ansible playbook**.
- Use cases:
  - Monolithic VM-based
  - Legacy applications or bare-metal servers
  - Teams needing full control over server configuration
  - Set up base OS for K8S

## Why Use Ansible?

- IT professionals often perform **repetitive tasks**:
  - Creating/sizing new hosts or VMs.
  - Applying configurations.
  - Patching servers.
  - Migrations and application deployments.
  - Security and compliance audits.
- These tasks require:
  - Executing hundreds of commands across many servers.
  - Maintaining the correct **sequence of events** (including reboots).
- Ansible simplifies and automates these repetitive workflows.

## Key Features

- **Automation without coding skills** – no need to be a programmer.
- **Minimal maintenance** compared to custom scripts.
- **Highly scalable** – can target:
  - A single host.
  - Groups of servers (e.g., web servers, database servers).
  - Hybrid environments (cloud + on-prem).
- **Flexible** – just modify one line to change the target environment.

## Example Use Cases

### 1. Restarting Hosts in a Specific Order

- Example scenario:
  - Power down **web servers** first.
  - Then power down **database servers**.
  - Power up **database servers** first.
  - Then power up **web servers**.
- With Ansible:
  - Create a playbook once.
  - Reuse it every time you need to restart the application stack.

### 2. Setting Up a Complex Infrastructure

- Provision hundreds of VMs across **public (e.g., AWS)** and **private clouds (e.g., VMware)**.
- Configure applications and communication between them:
  - Modify configuration files.
  - Install applications.
  - Configure firewall rules.
- Leverage **Ansible’s built-in modules** for these operations.

## Integrations

- **Dynamic inventory** – pull data from external sources (e.g., CMDB) to determine which VMs to target.
- **Event-driven automation** – trigger Ansible playbooks automatically from tools like **ServiceNow** when workflows are approved.

## Learning Resources

- **Official documentation**: [Ansible.com](https://www.ansible.com)
  - Comprehensive guides.
  - Hundreds of playbook examples.
- Upcoming course exercises will cover playbook creation and syntax step by step.
