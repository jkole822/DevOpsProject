# Terraform Basics: HCL, Resource Blocks & Workflow

## 1. Understanding HCL (HashiCorp Configuration Language)

- **Terraform uses HCL (not SQL)** for configuration.
- **Structure of a `.tf` File:**
  - Consists of **blocks** and **arguments**.
  - **Block:**
    - Defined within `{ }` (curly braces).
    - Represents a unit of configuration (e.g., resource, provider, variable).
  - **Arguments:**
    - Written in `key = value` format.
    - Provide configuration data inside a block.

## 2. Resource Block Structure

A resource block is the most basic and essential block in Terraform.

**Example: Creating a Local File**

```hcl
resource "local_file" "pet" {
  filename = "/root/pet.txt"
  content  = "We love pets"
}
```

### Breakdown:

1. `resource` keyword – identifies this as a resource block.
2. Resource Type:
   - `local_file` → indicates:
     - Provider: `local`
     - Resource type: `file`
3. Resource Name:

- `pet` → logical name used to refer to this resource within Terraform.

4. Arguments:
   - `filename` → absolute path of the file to create.
   - `content` → data to be written inside the file.
     > **Note**: The arguments `filename` and `content` are specific to the `local_file` resource and cannot be renamed. Each resource type has its own set of required and optional arguments.

## 3. More Resource Examples

- AWS EC2 Instance:

```hcl
resource "aws_instance" "web_server" {
ami           = "ami-12345678"
instance_type = "t2.micro"
}
```

- AWS S3 Bucket:

```hcl
resource "aws_s3_bucket" "data" {
bucket = "my-data-bucket"
acl    = "private"
}
```

## 4. Terraform Workflow (4-Step Process)

1. Write the configuration file (`.tf`).
2. Initialize the working directory:

```hcl
terraform init
```

- Downloads provider plugins (e.g., `local` provider).

3. Review the execution plan:

```hcl
terraform plan
```

- Displays what Terraform will do (like a `git diff`).
- Resources to be created are marked with a +.
- Safe step to verify before making changes.

4. Apply the configuration:

```hcl
terraform apply
```

- Shows plan again and asks for confirmation (`yes`).
- Creates the actual infrastructure.

## 5. Validating & Inspecting

- View Created Resource:

```hcl
cat /root/pet.txt
```

- Inspect Terraform State:

```hcl
terraform show
```

- Displays details of resources from the state file.

## 6. Providers & Documentation

- Terraform supports 100+ providers (AWS, Azure, GCP, Local, etc.).
- Each provider has:
  - A list of resource types.
  - Each resource type has:
    - Required arguments
    - Optional arguments
- Terraform Documentation is the single source of truth:
  - Use it to check available resource types.
  - Find all arguments and their descriptions.
  - Example: `local` provider has only one resource → `local_file`.

## 7. Key Takeaways

- Blocks are the building blocks of Terraform configuration.
- Resource blocks are mandatory to create infrastructure.
- Arguments must follow resource documentation.
- Terraform workflow: `init → plan → apply → show`
- Always refer to the official Terraform docs for providers, resources, and argument details.
