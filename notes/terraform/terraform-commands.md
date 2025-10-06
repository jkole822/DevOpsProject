# Terraform Commands — Notes

Until now, we have seen quite a few Terraform commands in action, such as terraform init, plan, and apply.
Let’s now take a look at some additional Terraform commands and their purposes.

## 1. terraform validate

**Purpose**:
Checks whether the configuration files are syntactically valid and internally consistent.

**Usage:**

```bash
terraform validate
```

**Output:**

- If everything is correct, Terraform displays a successful validation message.
- If there are errors, it shows the line causing the issue with hints to fix it.

**Example Error (HCL):**

```hcl
# Incorrect argument
file permissions = "0644"
# Correct argument
file_permission = "0644"
```

## 2. terraform fmt (Format)

**Purpose:**
Formats the configuration files in the current working directory into a canonical format.

**Usage:**

```bash
terraform fmt
```

**Benefits:**

- Improves code readability.
- Displays the files that were reformatted on screen.

## 3. terraform show

**Purpose:**

- Prints out the current state of the infrastructure as seen by Terraform.

**Usage:**

```bash
terraform show
terraform show -json
```

**Details Displayed:**

- Resource attributes such as file name, permissions, content, and ID.
- Tip:
    - Use the -json flag to print the state output in JSON format.

## 4. terraform providers

**Purpose:**

- Lists all providers used in the current configuration directory.

**Usage:**

```bash
terraform providers
```

**Subcommand (mirror):**

- To copy provider plugins needed for the current configuration to another directory:

```bash
terraform providers mirror /root/terraform/new_local_file
```

## 5. terraform output

**Purpose:**

- Prints all output variables defined in the configuration.

**Usage:**

```bash
terraform output
terraform output <variable_name>
```

**Example:**

```bash
terraform output my_variable
```

## 6. terraform refresh

**Purpose:**

- Synchronizes Terraform state with the real-world infrastructure.

**Usage:**

```bash
terraform refresh
```

**Behavior:**

- Updates the state file if resources have changed outside Terraform’s control.
- Does not modify actual infrastructure resources.
- Note:
    - Terraform automatically runs refresh during plan and apply, but you can disable it with:

```bash
terraform plan -refresh=false
```

## 7. terraform graph

**Purpose:**

- Generates a visual representation of resource dependencies in the configuration or execution plan.
  **Usage:**

```bash
terraform graph
```

**Output:**

- Produces text in the DOT graph format.
- Visualization (Graphviz):

1. Install Graphviz (Ubuntu):

```bash
sudo apt install graphviz
```

2. Generate the graph (PNG):

```bash
terraform graph | dot -Tpng > graph.png
```

3. View the graph:

- Open graph.png in your browser or image viewer.
  **Example Structure:**
- Root: Configuration directory.
- Resources:
    - local_file.pet
    - random_pet.my_pet
- Relationship:
    - local_file.pet depends on random_pet.my_pet.

## Summary

In this lecture, we learned the following Terraform commands:

| Command | Description |
| --------------------- | ---------------------------------------- |
| `terraform validate`  | Validates configuration syntax and logic |
| `terraform fmt`       | Formats configuration files |
| `terraform show`      | Displays the current state |
| `terraform providers` | Lists or mirrors providers |
| `terraform output`    | Displays output variables |
| `terraform refresh`   | Updates state with real-world changes |
| `terraform graph`     | Visualizes resource dependencies |
