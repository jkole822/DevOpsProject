# Terraform Data Sources

## Overview

In this section, we explore **data sources** in Terraform — how they work and why they’re used.

Terraform normally uses **configuration files** and a **state file** to provision and manage infrastructure resources.  
However, not all infrastructure is created or managed by Terraform. Some resources may be:

- Manually provisioned
- Created by other tools (e.g., **Puppet**, **CloudFormation**, **Ansible**, **SaltStack**)
- Managed by Terraform in another configuration or workspace

Data sources allow Terraform to **read information** from these existing resources and use that information within
Terraform-managed infrastructure.

---

## Why Use Data Sources?

Imagine a scenario where:

- A **database instance** is created manually in the cloud.
- You want Terraform to provision an **application server** that connects to this database.

Even though Terraform doesn’t manage the database resource directly, it can **read attributes** such as:

- Database name
- Host address
- Username

This is achieved through **data sources**.

---

## Example: Local File Data Source

### Scenario

We already have a managed local file resource in Terraform:

```hcl
resource "local_file" "pet" {
  filename = "/root/pet.txt"
  content  = "We love pets"
}
```

Terraform creates this file and tracks it in the state file.

Now, suppose we create another file manually (outside Terraform) using a shell script:

```bash
echo "Dogs are awesome" > /root/dogs.txt
```

This new file is not managed by Terraform, but we want to use its content inside our Terraform-managed file.

## Defining a Data Source

We can read the attributes of the manually created file using a data source block:

```hcl
data "local_file" "dog" {
    filename = "/root/dogs.txt"
}
```

**Explanation**

- The block starts with the keyword data (instead of resource).
- The first argument (local_file) specifies the data source type — in this case, a file.
- The second argument (dog) is a logical name for this data source.
- Inside the block, we define arguments specific to that data source (here: filename).

## Using Data from the Data Source

We can now use attributes from the data source inside our existing Terraform resource:

```hcl
resource "local_file" "pet" {
    filename = "/root/pet.txt"
    content  = data.local_file.dog.content
}
```

**How It Works**

- Terraform reads `/root/dogs.txt` (the unmanaged file).
- The content of that file (`dogs are awesome`) is assigned to `/root/pet.txt`.
- Effectively, data from outside Terraform is now integrated into a Terraform-managed resource.

## Data Source Attributes

Each data source exposes specific attributes.

For the `local_file` data source, the key attributes are:

| Attribute        | Description                                |
|------------------|--------------------------------------------|
| `content`        | Raw text content of the file               |
| `content_base64` | Base64-encoded version of the file content |

You can find this information in the Terraform Registry documentation for each provider under:
➡️ Data Sources → __Attributes Exported__

## Comparison: Resources vs Data Sources

| Feature    | Resource                                  | Data Source                              |
|------------|-------------------------------------------|------------------------------------------|
| Keyword    | `resource`                                | `data`                                   |
| Purpose    | Create, update, or destroy infrastructure | Read information from existing resources |
| Management | Managed by Terraform                      | External or read-only                    |
| Example    | `resource "aws_instance" "web"`           | `data "aws_ami" "ubuntu"`                |

- Resources are often called managed resources.
- Data sources are sometimes referred to as data resources.

## Summary

- Data sources allow Terraform to read information from existing infrastructure.
- They are useful when:
    - Some resources are created manually or by other tools.
    - You want Terraform-managed resources to reference external data.
- Defined using the data block syntax:

```hcl
data "<provider>_<type>" "<name>" {
    # arguments
}
```

- Accessed via the data.<provider>_<type>.<name>.<attribute> format.