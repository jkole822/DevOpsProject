# Variables in Ansible

## What are Variables?

- Variables in Ansible work just like variables in any scripting or programming language.
- They are used to **store values that vary** across different items.

**Example:**

- Applying patches to hundreds of servers:
  - A single playbook can be used for all servers.
  - Variables store **different hostnames, usernames, or passwords** for each server.

## Variables in Inventory

- We've already seen variables in the **inventory file**:
  - `ansible_host`
  - `ansible_connection`
  - `ansible_ssh_pass`
- **Inventory variables** help manage server-specific configurations.

## Defining Variables

### Inside Playbooks

- Variables can be defined directly in a playbook using the `vars` directive.
- **Example:**

```yaml
vars:
  dns_server: 192.168.1.1
```

### Separate Variable Files

- Variables can also be stored in dedicated files for better organization.
- Useful when working with includes and roles.

## Using Variables

- To use a variable in a playbook, reference it with double curly braces:

```yaml
nameserver: "{{ dns_server }}"
```

- Ansible will replace the variable with its value when the playbook runs.
  **Benefits:**
- Makes playbooks reusable.
- Reduces the need to modify playbooks; update only the inventory or variable file.

## Example: Firewall Playbook

- Original playbook: hardcoded values.
- Problem: Hardcoded values make it less reusable.
- Solution:
  - Move variable values to the inventory or host-specific variable files.
  - Use double curly braces (`{{ }}`) in the playbook.
  - This allows playbook reuse without modification.

## Host Variable Files

- Host-specific variable files store variables for a particular host.
- Example: `web.yml` for the `web` host.
- All variables in this file are available to the playbook for that host.

## Jinja2 Templating

- Format used to reference variables in playbooks is called Jinja2 templating.
- Tips:
  - Enclose variables in double curly braces (`{{ variable_name }}`).
  - If a variable starts a sentence, use code formatting.
  - If a variable is mid-sentence, code formatting is optional.
