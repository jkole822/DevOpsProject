# Ansible Plugins Notes

## Overview

- **Plugins** are pieces of code that augment Ansible’s core functionality.
- Unlike modules, **plugins do not perform tasks on remote hosts**.
- They enhance the behavior of **playbooks, tasks, inventories, and outputs**.

## Types of Ansible Plugins

### 1. Callback Plugins

- Triggered at various points during playbook execution.
- Examples:
  - `default` → standard output
  - `json` → output in JSON format
  - `timer` → shows total playbook execution time
  - `mail` → send email notifications
- Usage:

```yaml
ansible.cfg:
  [defaults]
  callback_whitelist = timer
```

### 2. Connection Plugins

- Control how Ansible connects to remote hosts.
- Examples:
  - `ssh` → default connection method
  - `paramiko` → alternative SSH implementation
  - `local` → run tasks on the local machine
  - `winrm` → connect to Windows hosts
- Can be set per-playbook or in inventory.

### 3. Inventory Plugins

- Dynamically generate inventory of hosts.
- Examples:
  - `ini` → classic INI inventory
  - `yaml` → YAML-based inventory
  - `script` → generate inventory via custom scripts
  - Cloud-specific plugins: `ec2`, `gce`, `openstack`
- Useful for dynamic environments like cloud infrastructures.
- To list all hosts in your AWS inventory using a dynamic inventory script, you generally use the --list option with the inventory script.
- Assuming your script is called aws_inventory.py, the command would be:

```bash
ansible-inventory ./aws_inventory.py --list
```

#### Explanation:

- `./aws_inventory.py` → runs your dynamic inventory script.
- `--list` → tells the script to output all hosts and groups in JSON format that Ansible can use as an inventory.
- The output will include:
  - All EC2 instances or resources defined in your AWS account
  - Groups, host variables, and metadata

#### Optional:

- If you want to see details for a specific host, you can use:

```bash
./aws_inventory.py --host <hostname>
```

- This is the standard way dynamic inventory scripts work in Ansible.

### 4. Lookup Plugins

- Fetch data from external sources or files to use in playbooks.
- Examples:
  - `file` → read content from a file
  - `env `→ read environment variables
  - `pipe` → fetch output from a shell command
- Example:

```yaml
- name: Read password from file
  debug:
    msg: "{{ lookup('file', '/tmp/password.txt') }}"
```

### 5. Filter Plugins

- Transform data inside Jinja2 templates or variable expressions.
- Examples:
  - `lower`, `upper` → string case conversion
  - `replace` → replace substrings
  - `default` → provide fallback value
- Example:

```yaml
debug:
  msg: "{{ 'Hello WORLD' | lower }}"
```

### 6. Strategy Plugins

- Control how tasks are executed on multiple hosts.
- Examples:
  - `linear` → default, tasks run sequentially
  - `free` → tasks run asynchronously on hosts
  - `host_pinned` → pins tasks to specific hosts
- Can improve playbook performance in large environments.

### 7. Action Plugins

- Extend how modules execute tasks.
- Typically used to wrap module execution with extra logic.
- Examples:
  - `assemble` → combines multiple files
  - `include_vars` → include variables dynamically

### 8. Vars Plugins

- Load variables dynamically from external sources.
- Examples:
  - `env` → load variables from environment
  - `host_group` → load variables per host/group

## Key Notes

- Plugins are core to customizing Ansible behavior.
- They do not run tasks on remote hosts (modules do that).
- Configured via:
  - `ansible.cfg`
  - Playbook directives
  - Inventory or environment variables
- Ansible provides many built-in plugins, but you can also create custom plugins.

## Useful Commands

- List all available plugins of a type:

```yaml
ansible-doc -t callback
ansible-doc -t connection
ansible-doc -t inventory
```

- Test a plugin: Refer to its documentation via `ansible-doc <plugin_name>`.
