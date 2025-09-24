# Ansible Loops

## Introduction

- Loops allow a single task to be executed **multiple times** with different values.
- Useful when performing repetitive tasks like creating multiple users.

## 1. Basic Loop with a List of Strings

- Example: Creating multiple users with the `user` module.
- Instead of duplicating tasks for each user, use a loop:

```yaml
- name: Create multiple users
  user:
    name: "{{ item }}"
    state: present
  loop:
    - Joe
    - George
    - Ravi
    - Sarah
```

- Explanation:
  - `loop` iterates over each item in the list.
  - `item` stores the current value for each iteration.
  - Reduces repetition and keeps playbooks organized.
- Visualization:
  - Each loop iteration behaves like a separate task:

```yaml
Task 1: item = Joe
Task 2: item = George
Task 3: item = Ravi
```

## 2. Loop with a List of Dictionaries

- When more complex data is required (e.g., username + UID):

```yaml
users:
  - name: Joe
    uid: 1010
  - name: George
    uid: 1020
  - name: Ravi
    uid: 1030
```

- Task using dictionary items:

```yaml
- name: Create users with UID
  user:
    name: "{{ item.name }}"
    uid: "{{ item.uid }}"
    state: present
  loop: "{{ users }}"
```

- Explanation:
  - `item` now represents a dictionary.
  - Access dictionary values using `item.key` (e.g., `item.name`, `item.uid`).
  - Visualize each iteration as a separate task with its own dictionary.

## 3. Alternative: with_items Directive

- Legacy syntax before loop was introduced:

```yaml
- name: Create multiple users
  user:
    name: "{{ item }}"
    state: present
  with_items:
    - Joe
    - George
    - Ravi
```

- Both `loop` and `with_items` yield the same result.
- Recommended to use `loop` for new playbooks, but understand `with_items` for legacy playbooks.

  ## 4. Other `with_` Directives

- Specialized looping directives exist for various data sources: - `with_files` → iterate over files
  - `with_url` → iterate over URLs
  - `with_mongodb` → iterate over MongoDB databases
- All `with_` directives use lookup plugins internally.
  - Lookup plugins are scripts that fetch or process data (files, URLs, databases, Kubernetes, etc.).

## Summary

- Loops reduce repetition and simplify playbooks.
- Can iterate over:
  - List of strings
  - List of dictionaries (complex data)
- Use `item` to access the current element in the loop.
- Modern syntax: `loop`
- Legacy syntax: `with_items` and other `with_` directives
- Lookup plugins expand looping capabilities to external systems.
