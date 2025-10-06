# Terraform Lifecycle Rules

## Overview

In this section, we learn how to **set up lifecycle rules in Terraform** to control how resources are created, updated,
or destroyed.

By default, Terraform enforces **immutability** — when updating a resource, it:

1. **Deletes** the existing resource.
2. **Creates** a new one with the updated configuration.

However, this default behavior may not always be desirable. Lifecycle rules give us fine-grained control over this
process.

## What Are Lifecycle Rules?

- **Lifecycle rules** modify the way Terraform handles changes to resources.
- They are defined **inside a `lifecycle` block**, which itself is placed **inside a resource block**.
- Syntax example:

```hcl
resource "example_resource" "example" {
  # resource configuration here

  lifecycle {
    # lifecycle rules go here
  }
}
```

## 1. `create_before_destroy`

**Purpose**

- Ensures Terraform creates a new resource before destroying the old one.

**Example**

```hcl
resource "local_file" "example" {
    filename = "example.txt"
    content  = "Hello World"

    lifecycle {
        create_before_destroy = true
    }
}
```

**Behavior**

- When configuration changes force a recreation, Terraform will:
    - Create the new resource first.
    - Destroy the old one afterward.
- Useful for avoiding downtime or data loss during updates.

**Example**

```hcl
resource "local_file" "file" {
    filename = var.filename
    file_permission =  var.permission
    content = "This is a random string - ${random_string.string.id}"
   
    lifecycle {
        create_before_destroy =  true
    }
}

resource "random_string" "string" {
    length = var.length
    keepers = {
        length = var.length
    }
    
    lifecycle {
        create_before_destroy =  true
    }
}
```

- `create_before_destroy` lifecycle rule with the `local_file` resource causes Terraform to attempt to create the new
  file first. However, if the filename argument is the same, Terraform will immediately delete the existing file before
  the new one can be created during the recreation process.
    - This illustrates why using `create_before_destroy` with `local_file` resources is not always advisable, as the
      file path must be unique for simultaneous creation.
    - On the other hand, the `random_string` resource is only recorded in Terraform state and does not have this
      limitation.

## 2. `prevent_destroy`

**Purpose**

- Prevents Terraform from destroying a resource for any reason.

**Example**

```hcl
resource "aws_db_instance" "production" {
    identifier = "prod-db"
    instance_class = "db.t3.micro"

    lifecycle {
        prevent_destroy = true
    }
}
```

**Behavior**

- If a configuration change would require deletion, Terraform throws an error and stops the plan.
- Helps protect critical resources (e.g., databases) from accidental deletion.
- ⚠️ Note:
    - This does not prevent deletion via the terraform destroy command — it only blocks deletion triggered by terraform
      apply.

## 3. ignore_changes

**Purpose**

- Instructs Terraform to ignore updates to specific resource attributes.
- Example Scenario: Consider an AWS EC2 instance defined as follows:

```hcl
resource "aws_instance" "web_server" {
    ami           = "ami-123456"
    instance_type = "t2.micro"

    tags = {
        Name = "ProjectA-WebServer"
    }

    lifecycle {
        ignore_changes = [tags]
    }
}
```

**Behavior**

- If the EC2 tag is manually changed outside Terraform (e.g., via AWS console or CLI), Terraform will not revert it on
  the next apply.
- Terraform ignores modifications to attributes listed under ignore_changes.
- Notes
    - ignore_changes accepts a list of attributes:

```hcl
lifecycle {
    ignore_changes = [tags, instance_type]
}
```

- To ignore all attributes:

```hcl
lifecycle {
    ignore_changes = all
}
```

- Useful when:
    - Other tools manage parts of the configuration.
    - Certain attributes are expected to change dynamically.

## Summary of Lifecycle Rules

| Rule                        | Description                                        | Use Case                              |
|-----------------------------|----------------------------------------------------|---------------------------------------|
| **`create_before_destroy`** | Creates new resource before destroying the old one | Prevent downtime during updates       |
| **`prevent_destroy`**       | Prevents accidental deletion of critical resources | Protect databases, persistent storage |
| **`ignore_changes`**        | Ignores external or manual changes to attributes   | Avoid unnecessary Terraform updates   |
