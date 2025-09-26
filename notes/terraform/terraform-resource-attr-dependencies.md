# Linking Resources and Resource Dependencies in Terraform

## 1. Using Resource Attributes to Link Resources

- Terraform allows **one resource to use the output of another**.
- Example scenario:
  - `local_file` resource:
    - Arguments: `filename`, `content`
  - `random_pet` resource:
    - Arguments: `prefix`, `separator`, `length`
  - Goal: Use the **random pet name** as the content of the local file.

### 1.1 Reference Expression & Interpolation

- Terraform syntax to reference another resource:

```
${resource_type.resource_name.attribute}
```

- Example:

```hcl
resource "local_file" "pet_file" {
  filename = "/root/pet.txt"
  content  = "My favorite pet is ${random_pet.my_pet.id}"
}
```

- Explanation:
  - `random_pet` → resource type
  - `my_pet` → resource name
  - `id` → attribute of `random_pet` (generated pet name)
  - `${}` → interpolation sequence to evaluate expression and insert into string

## 2. Implicit Dependency

- When using reference expressions, Terraform automatically determines resource creation order.
- In the example above:
  1. Terraform creates the `random_pet` resource first.
  2. Terraform then creates the `local_file` resource using the generated `id`.
- Deletion occurs in reverse order:
  1. `local_file` deleted first
  2. `random_pet` deleted next
- This is called an implicit dependency because Terraform figures it out automatically.

## 3. Explicit Dependency

- Sometimes resources do not directly reference each other but still require a specific order.
- Use the `depends_on` argument to specify explicit dependencies:

```hcl
resource "local_file" "pet_file" {
  filename = "/root/pet.txt"
  content  = "Static content"

  depends_on = [
    random_pet.my_pet
  ]
}
```

- Effect:
  - Ensures `local_file` is created after `random_pet`.
- Useful when:
  - Dependencies are indirect.
  - No reference expressions are used in the dependent resource.

## 4. Key Takeaways

- Reference expressions create implicit dependencies automatically.
- depends_on argument allows explicit dependency specification.
- Terraform ensures resources are:
  - Created in the correct order
  - Destroyed in the reverse order
- Explicit dependencies are only necessary when a resource relies indirectly on another resource.
