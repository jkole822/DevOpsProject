# Output Variables in Terraform

## 1. What Are Output Variables?

- **Output variables** allow you to **store and display the result of an expression** after Terraform execution.
- Useful for:
  - Displaying information about provisioned resources
  - Passing values to other tools (scripts, Ansible playbooks, etc.)

## 2. Syntax for Output Variables

```hcl
output "pet_name" {
  description = "The randomly generated pet name"
  value       = random_pet.my_pet.id
}
```

**Key Points:**

- `output` → Keyword to define output block
- `pet_name` → Logical name of the output variable
- `value` → Mandatory argument that stores a reference expression
- `description` → (Optional) Helps document what this output variable represents

## 3. Viewing Output Variables

**After running `terraform apply`:**

- Terraform displays output variables automatically.
- To view them manually:

```bash
terraform output          # Displays all outputs
terraform output pet_name # Displays value of specific output variable
```

## 4. Use Cases

- Quickly display key details of provisioned resources (e.g., IP address, instance ID)
- Pass values to external tools for:
  - Configuration management
  - Testing
  - Automation scripts

## 5. Key Notes

- Output variables are not necessary for inter-resource dependencies
  (those are handled using reference expressions directly).
- They are best used for:
  - Human-readable results
  - Integrations with external systems
