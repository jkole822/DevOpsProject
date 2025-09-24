# Ansible Playbooks Notes

## What are Ansible Playbooks?

- Playbooks are **Ansible’s orchestration language**.
- They define **what we want Ansible to do**.
- Essentially, a playbook is a **set of instructions** provided to Ansible to automate tasks.

### Examples of tasks:

- Running a series of commands on multiple servers in sequence.
- Restarting servers in a particular order.
- Deploying hundreds of VMs in public/private clouds.
- Provisioning storage, setting up networks and clusters.
- Configuring applications like web servers or databases.
- Setting up load balancing, monitoring, backup clients.
- Updating configuration databases with new VM information.

## Playbook Structure

- Written in **YAML format**.
- **Single YAML file** containing one or more **plays**.
- **Play:** Defines a set of activities to run on a host or group of hosts.
- **Task:** Single action performed on a host (e.g., run a command, install a package, restart a service).

### YAML Concepts Recap

- A playbook is a **list of plays** (denoted by `-` in YAML).
- Each play is a **dictionary** with properties:
  - `name` – Name of the play
  - `hosts` – Target host or group
  - `tasks` – Ordered list of actions
- **Tasks are ordered**; sequence matters (e.g., install HTTP service **before** starting the web server).

## Sample Playbook (Conceptual)

- Play: `Play one`
- Host: `localhost`
- Tasks:
  1. Print the date
  2. Run a script on the server
  3. Install HTTP package (`yum` module)
  4. Start web server (`service` module)

```yaml
- name: Play 1
  hosts: localhost
  tasks:
    - name: Execute command 'date'
      command: date
    - name: Execute script on server
      script: /path/to/your/script.sh
    - name: Install httpd service
      yum:
        name: httpd
        state: present
    - name: Start web server
      service:
        name: httpd
        state: started
```

### Key Points

- Host is defined at the **play level**.
- Hosts/groups must match the inventory file.
- Tasks are executed in the order listed.
- YAML indentation and structure are **critical**.

## Hosts and Inventory

- Host parameter specifies **where tasks run**.
- Can target a single host or a group of hosts.
- Connection info is retrieved from the **inventory file**.
- Groups execute tasks **simultaneously on all members**.

## Ansible Modules

- Tasks use **modules** to perform actions.
- Examples: `command`, `script`, `yum`, `service`.
- Hundreds of modules are available out-of-the-box.
- Documentation: [Ansible Modules](https://docs.ansible.com)
- Command to explore modules:

```bash
ansible-doc -l
```

## Running Playbooks

- Use the `ansible-playbook` command:

```bash
ansible-playbook your_playbook.yml
```

- To explore additional parameters:

```bash
ansible-playbook --help
```

- To run in check mode:

```bash
ansible-playbook playbook.yml --check
```

- Simulates the playbook run without making any changes, showing what would change.
- Optional flags:
  - `--diff` → shows the differences that would be made to files.
  - `-i inventory` → specify a custom inventory file.
- So, a full dry-run command could look like:

```bash
ansible-playbook -i inventory playbook.yml --check --diff
```

- To run a syntax check:

```bash
ansible-playbook playbook.yml --syntax-check
```

- Checks the YAML syntax and basic playbook structure without executing any tasks.
- This is useful to catch **YAML formatting errors, missing colons, indentation problems, or incorrect module usage** before running the playbook.

- To lint a playbook:

```bash
ansible-lint playbook.yml
```

- It checks for things like:
  - YAML formatting issues
  - Deprecated modules or parameters
  - Hard-coded passwords
  - Inefficient or non-idempotent tasks
  - Best-practice violations
- Optional flags:
  - `-v` → verbose output.
  - `-r RULES_DIR` → use custom rules directory.
  - `--exclude` → exclude files or directories from linting.

## Summary

- Playbooks define tasks in YAML.
- Plays define tasks for hosts or groups.
- Tasks are ordered actions executed via modules.
- Proper YAML syntax and structure are crucial.
- Inventory file links hosts/groups to playbooks.
