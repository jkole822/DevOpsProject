# Mutable vs Immutable Infrastructure

## Overview

In this section, we learn about the **difference between mutable and immutable infrastructure** and why Terraform
destroys and recreates resources when applying changes.

---

## Mutable Infrastructure

### Example Scenario

- Consider an application server running **Engine X v1.17**.
- When a new version (e.g., **v1.18** or **v1.19**) is released, the server is **updated in place**.
- The same server hardware and configuration remain; only the **software or configuration changes**.

### How Updates Are Done

- Updates can be performed:
    - Manually (e.g., downloading and upgrading during a maintenance window)
    - Using automation tools like **Ansible** or **custom scripts**
- For **high availability**, multiple servers run the same software, and each must be upgraded individually.

### Characteristics

- **In-place updates** modify existing systems directly.
- The **underlying infrastructure remains unchanged**.
- This approach is called **mutable infrastructure**.

---

## Configuration Drift

### What Is Configuration Drift?

- Over time, differences may appear among servers due to:
    - Failed upgrades
    - Missing dependencies
    - Network or system inconsistencies
- For example:
    - Web Server 1 and 2 → Engine X v1.19
    - Web Server 3 → Engine X v1.18
    - Different OS versions or configurations across servers

### Problems with Configuration Drift

- Leads to **inconsistent environments**
- Makes **troubleshooting difficult**
- Complicates **future updates and maintenance**

---

## Immutable Infrastructure

### Concept

- Instead of upgrading existing servers, **new servers are created** with the updated version.
- The **old servers are deleted** after the new ones are verified to work correctly.
- Example:
    - Replace Engine X v1.17 server with a new one running v1.18.

### Characteristics

- **Immutable** = “unchangeable”
- No **in-place updates**
- Each update involves **creating new resources** and **destroying old ones**

### Benefits

- **Eliminates configuration drift**
- Ensures **consistent and predictable environments**
- Simplifies **versioning**, **rollback**, and **rollforward**
- Easier to manage with **Infrastructure as Code (IaC)** tools like Terraform

---

## Terraform’s Approach

### Why Terraform Recreates Resources

- Terraform treats resources as **immutable by default**.
- When a change is made (e.g., file permissions from `777` to `700`), Terraform:
    1. **Destroys** the old resource.
    2. **Creates** a new resource with the updated configuration.

### Lifecycle Customization

- Terraform allows customizing this behavior using **lifecycle rules** in the resource block.
- Lifecycle options can:
    - **Create before destroy**
    - **Prevent deletion**
    - **Ignore changes** to specific attributes

---

## Summary

| Feature        | Mutable Infrastructure    | Immutable Infrastructure           |
|----------------|---------------------------|------------------------------------|
| Update Method  | In-place updates          | Replace with new instance          |
| Infrastructure | Remains the same          | Recreated each update              |
| Risk of Drift  | High                      | Very low                           |
| Rollback       | Complex                   | Simple (recreate previous version) |
| Example        | Manual or Ansible updates | Terraform recreate approach        |

