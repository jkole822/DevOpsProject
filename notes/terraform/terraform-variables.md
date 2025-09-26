# Using Variables in Terraform

## 1. Why Use Variables?

- **Problem:** Hardcoding values directly in `.tf` files reduces reusability and flexibility.
- **Goal:** Make configuration **dynamic** by using **input variables**.
- **Benefit:** Same code can be reused for multiple deployments with different values.

## 2. Defining Variables

Create a `variables.tf` file:

```hcl
variable "filename" {
  default = "/root/pet.txt"
}

variable "content" {
  default = "We love pets"
}

variable "prefix" {
  default = "dog"
}

variable "separator" {
  default = "-"
}

variable "length" {
  default = 1
}
```

**Breakdown:**

- Keyword: `variable` → defines a variable block.
- Variable name: Descriptive name for the variable (usually matches the argument name).
- Default value: (Optional) assigns a default value.
  - If omitted, the value must be provided at runtime.

## 3. Using Variables in Main Configuration

- In main.tf, reference variables like this:

```hcl
resource "local_file" "pet" {
    filename = var.filename
    content  = var.content
}

resource "random_pet" "my_pet" {
    prefix    = var.prefix
    separator = var.separator
    length    = var.length
}
```

**Notes:**

- Use `var.<variable_name>` syntax to reference variables.
- No need to wrap `var.<name>` in quotes.

## 4. Updating Resource Values

- Simply update values in `variables.tf`.
- No need to modify `main.tf`.

**Example Update:**

```hcl
variable "content" {
    default = "My favorite pet is Mrs. Whiskers"
}

variable "length" {
    default = 2
}
```

When running:

```bash
terraform apply
```

Terraform will:

- Recreate resources if required.
- Update file content.
- Generate a new random pet name with two words.

## 5. Example: AWS EC2 Instance with Variables

```hcl
variable "ami_id" {
    default = "ami-12345678"
}

variable "instance_type" {
    default = "t2.micro"
}

resource "aws_instance" "web" {
    ami           = var.ami_id
    instance_type = var.instance_type
}
```

This allows easy updates by changing only `variables.tf` — no need to edit the main configuration file.

## 6. Key Takeaways

- Variables improve reusability and maintainability.
- Place them in a separate `variables.tf` file for better organization.
- Use `var.<name>` syntax inside resource blocks.
- Changing variable values allows quick updates without touching `main.tf`.

# Terraform Variables – Deep Dive

## 1. Variable Block & Its Arguments

A variable block in Terraform can use three arguments:

1. **default** – (Optional) specifies the default value.
2. **type** – (Optional) enforces the type of the variable.
   - If omitted, defaults to `any`.
3. **description** – (Optional) adds context about the variable.
   - Good practice to document purpose and usage.

## 2. Basic Variable Types

| Type     | Description                          | Example          |
| -------- | ------------------------------------ | ---------------- |
| `string` | Single alphanumeric value            | `"We love pets"` |
| `number` | Numeric value (positive or negative) | `42`             |
| `bool`   | Boolean value                        | `true` / `false` |

If `type` is not specified, Terraform accepts any type.

## 3. Complex Variable Types

Terraform supports advanced types for structured data:

### a) List

- **Definition:** Ordered collection of elements.
- **Indexed:** Accessed by position (starting at index `0`).
- **Example:**

```hcl
variable "prefix" {
  type    = list(string)
  default = ["Mr.", "Mrs.", "Sir"]
}

# Accessing elements:
var.prefix[0] # "Mr."
var.prefix[1] # "Mrs."
var.prefix[2] # "Sir"
```

## b) Map

- Definition: Key-value pairs stored in `{}`.
- Example:

```hcl
variable "file_content" {
    type = map(string)
    default = {
        statement1 = "This is line one"
        statement2 = "This is line two"
    }
}

# Access a value:
var.file_content["statement2"] # "This is line two"
```

## c) Type Constraints with Lists & Maps

- You can specify element types explicitly:

```hcl
variable "numbers" {
    type    = list(number)
    default = [1, 2, 3]
}
```

- If default values do not match the type constraint, Terraform will throw an error.

## d) Set

- Similar to a list, but cannot contain duplicate elements.
- Example:

```hcl
variable "unique_ids" {
    type    = set(number)
    default = [1, 2, 3] # ✅ valid
}
```

- Invalid example (duplicate values):

```hcl
default = [1, 2, 2] # ❌ will cause an error
```

## e) Object

- **Definition**: Collection of named attributes with explicitly defined types.
- Example:

```hcl
variable "bella" {
    type = object({
        name         = string
        color        = string
        age          = number
        food         = list(string)
        favorite_pet = bool
    })
    default = {
        name         = "Bella"
        color        = "Brown"
        age          = 7
        food         = ["fish", "chicken", "turkey"]
        favorite_pet = true
    }
}
```

- **Usage**: Access attributes with dot notation:

```hcl
var.bella.name
var.bella.food[1]
```

## f) Tuple

- **Definition**: Ordered collection where element types are explicitly declared.
- Allows different types in a fixed-length sequence.
- Example:

```hcl
variable "pet_info" {
    type    = tuple([string, number, bool])
    default = ["cat", 7, true]
}
```

- Must have exactly three elements and match types in order.
- Example of incorrect usage:

```hcl
default = ["cat", 7, true, "dog"] # ❌ error - extra element
```

## 4. Key Takeaways

- Use `description` for clarity and documentation.
- Use `type` to enforce data integrity.
- Lists vs Sets:
  - Lists allow duplicates and preserve order.
  - Sets reject duplicates and do not guarantee order.
- Objects and tuples allow modeling complex structures.
- Type constraints help catch errors early during `terraform plan`.

# Ways to Use Input Variables

Terraform supports multiple ways to pass values to input variables.  
So far, we have assigned **default values** inside the `variable` block, but this is optional — and there are several other ways to provide values.

## 1. Interactive Input

If a variable block does **not** have a default value:

```hcl
variable "file_name" {
  type = string
}
```

Running `terraform apply` will prompt you to enter a value interactively:

```bash
var.file_name
  Enter a value: /root/pets.txt
```

## 2. Command-Line Flags

You can provide variables directly when running Terraform commands:

```bash
terraform apply -var="file_name=/root/pets.txt" -var="length=2"
```

- `-var` can be repeated multiple times for multiple variables.

## 3. Environment Variables

Terraform supports setting variables via environment variables:

```bash
export TF_VAR_file_name="/root/postcode"
export TF_VAR_length=2
terraform apply
```

- The naming convention is `TF_VAR_<variable_name>`.

## 4. Variable Definition Files

You can store variable values in files:

- Format: `.tfvars` or `.tfvars.json`
- Example:

```hcl
# terraform.tfvars
file_name = "/root/pets.txt"
length    = 2
```

Terraform automatically loads:

- `terraform.tfvars`
- `terraform.tfvars.json`
- Any file ending in `.auto.tfvars` or `.auto.tfvars.json`

For custom-named files:

```bash
terraform apply -var-file="myvars.tfvars"
```

## 5. Variable Definition Precedence

When the same variable is set in multiple places, Terraform uses the following order (lowest → highest priority):

1. Environment Variables
2. `terraform.tfvars` / `terraform.tfvars.json`
3. `*.auto.tfvars` or `*.auto.tfvars.json` files (alphabetical order if multiple files exist)
4. Command-line `-var` or `-var-file` flags (highest precedence)

**Example**:

- Environment variable: `/root/cats.txt`
- `terraform.tfvars`: `/root/snakes.txt`
- `variables.auto.tfvars`: `/root/tigers.txt`
- Command line: `-var="file_name=/root/bestpets.txt"`
  ✅ Final value used → `/root/bestpets.txt` (command-line flag wins)

## Key Takeaways

- Default values make variables optional but limit flexibility.
- Interactive input is useful for quick runs but not automation.
- Command-line `-var` flags are convenient but can get lengthy with many variables.
- Variable definition files are the best practice for managing multiple variables.
- Precedence matters when using multiple methods — the last method in the order wins.
