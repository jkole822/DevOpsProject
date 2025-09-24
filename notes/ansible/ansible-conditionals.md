# Ansible Conditionals

## Introduction

- Goal: Create a **single playbook** that works for multiple OS flavors.
- Problem: Different OS flavors use different package managers:
  - Debian → `apt`
  - RedHat → `yum`
- Solution: Use **conditional statements** (`when`) to execute tasks only if certain conditions are met.

---

## 1. Basic Conditional

- **Syntax:**

```yaml
when: <condition>
```

- Example: Run a task only for Debian or RedHat:

```yaml
- name: Install package on Debian
  apt:
    name: engineX
    state: present
  when: ansible_os_family == "Debian"

- name: Install package on RedHat
  yum:
    name: engineX
    state: present
  when: ansible_os_family == "RedHat"
```

- Notes:
  - Use `==` for equality.
  - `ansible_os_family` is a built-in variable populated by Ansible.

## 2. Using Logical Operators

- OR Operator:

```yaml
when: ansible_os_family == "RedHat" or ansible_os_family == "Suse"
```

- AND Operator:

```yaml
when: ansible_os_family == "Debian" and ansible_distribution_version == "16.04"
```

## 3. Conditionals in Loops

- Scenario: Install multiple packages only if required.
- Example:

```yaml
- name: Install required packages
  apt:
    name: "{{ item.name }}"
    state: present
  loop: "{{ packages }}"
  when: item.required == true
```

- Explanation:
  - `packages` is a list of dictionaries:

```yaml
packages:
  - name: pkg1
    required: true
  - name: pkg2
    required: false
  - name: pkg3
    required: true
```

- Each `item` contains the package details.
- Condition `when: item.required == true` ensures only required packages are installed.
- Visualize loop as multiple tasks, one for each item.

## 4. Conditional Based on Task Output

- Scenario: Check service status and send email if it’s down.
- Steps:

1. Check service status and register output:

```yaml
- name: Check service status
  command: systemctl status myservice
  register: result
```

2. Send email conditionally:

```yaml
- name: Send email if service is down
  mail:
    to: admin@example.com
    subject: "Service Status"
    body: "Service is down!"
  when: result.stdout.find("down") != -1
```

- Notes:
  - `register` captures output of a task.
  - `find` method returns:
    - Position of the string if found
    - `-1` if string is not found
  - Condition checks for presence of `down` in the output.

## Summary

- Use when to run tasks conditionally.
- Conditions can be based on:
  - OS type (`ansible_os_family`)
  - Logical operators (`and`, `or`)
  - Loop items (`item.required`)
  - Task outputs (`register` variable)
- Helps create flexible playbooks for multiple hosts or conditions.
