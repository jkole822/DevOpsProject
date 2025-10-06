# Terraform Meta Arguments: `count` and `for_each`

## Overview

In this lecture, we explore two important **looping meta arguments** in Terraform — `count` and `for_each`.  
These allow us to **create multiple instances** of a resource efficiently and programmatically.

---

## 1. The `count` Meta Argument

### Purpose

The `count` meta argument allows you to **create multiple instances** of the same resource by simply specifying a
number.

### Example: Creating Multiple Local Files

```hcl
resource "local_file" "pet" {
  filename = "/root/pet.txt"
  content  = "We love pets"
  count    = 3
}
```

**Result**

When you run:

```bash
terraform plan
terraform apply
```

Terraform creates three resources:

```
local_file.pet[0]
local_file.pet[1]
local_file.pet[2]
```

Each resource is indexed starting from `0`.

## Problem: Identical Resource Attributes

Since the `filename` is static, Terraform will attempt to create the same file three times, which leads to conflicts.

To fix this, we need to give each file a unique name.

## Using a List Variable for Dynamic Filenames

We can define a variable that stores multiple filenames:

```hcl
variable "filenames" {
  default = [
    "/root/sdcard.txt",
    "/root/dogs.txt",
    "/root/cats.txt"
  ]
}
```

Then, update the resource block to use the list elements dynamically:

```hcl
resource "local_file" "pet" {
  filename = var.filenames[count.index]
  content  = "Terraform loves pets!"
  count    = 3
}
```

- `count.index` provides the current loop index (0, 1, 2).
- Terraform now creates three distinct files:
    - /root/sdcard.txt
    - /root/dogs.txt
    - /root/cats.txt

## Making count Dynamic with the `length()` Function

If you later add more filenames to the list, Terraform won’t automatically increase the number of files unless you
update `count`.
To fix that, use the built-in `length()` function:

```hcl
resource "local_file" "pet" {
    filename = var.filenames[count.index]
    content  = "Terraform loves pets!"
    count    = length(var.filenames)
}
```

## Built-In Function Used

`length()` – returns the number of elements in a list, map, or set.

**Example:**

```hcl
length(["a", "b", "c"])  # returns 3
```

## 2. The Drawback of Using count

While count works, it can create unwanted resource replacements when the list changes.

**Example Issue**

Suppose we remove the first element (`/root/sdcard.txt`) from the list.

Terraform will interpret this as:

- Delete the first resource (`pet[0]`).
- Replace the next two resources (`pet[1]` and `pet[2]`), even though their contents are unchanged.

This happens because:

- Terraform stores resources as a list.
- List elements are identified by index, not by value.
- When one element is removed, all subsequent indexes shift, causing resource re-creation.

## Visualizing the Problem

| Index | Filename Before    | Filename After   | Terraform Action |
|-------|--------------------|------------------|------------------|
| [0]   | `/root/sdcard.txt` | `/root/dogs.txt` | Replace          |
| [1]   | `/root/dogs.txt`   | `/root/cats.txt` | Replace          |
| [2]   | `/root/cats.txt`   | *Removed*        | Delete           |

Even though we only removed one filename, Terraform plans to replace two and delete one.
This is not ideal for maintaining state stability.

## 3. Using `for_each` Instead of `count`

To solve this problem, we can use the `for_each` meta argument, which identifies resources by key instead of by index.

**Example**

```hcl
resource "local_file" "pet" {
  for_each = toset(var.filenames)
  filename = each.value
  content  = "Terraform loves pets!"
}
```

**Explanation**

- `for_each` expects a map or set.
- `each.value` refers to the current element in the loop.
- Terraform creates resources identified by keys, not by indexes.

Example resource identifiers:

```
local_file.pet["/root/sdcard.txt"]
local_file.pet["/root/dogs.txt"]
local_file.pet["/root/cats.txt"]
```

## Fixing the List Type

- If `filenames` is defined as a list, you’ll see an error:

> The "for_each" argument must be a map or set of strings.

You can fix this in two ways:

**Option 1: Change the variable type**

```hcl
variable "filenames" {
  type = set(string)
  default = [
    "/root/sdcard.txt",
    "/root/dogs.txt",
    "/root/cats.txt"
  ]
}
```

**Option 2: Convert the list to a set**

```hcl
resource "local_file" "pet" {
  for_each = toset(var.filenames)
  filename = each.value
  content  = "Terraform loves pets!"
}
```

## Behavior When Updating

If we remove `/root/sdcard.txt` from the variable:

- Terraform identifies resources by key, not by list index.
- Only the matching resource is destroyed.
- The remaining resources are left untouched.

This makes `for_each` safer and more predictable for managing dynamic lists.

## Visualizing the Difference

| Feature                    | `count`                     | `for_each`                       |
|----------------------------|-----------------------------|----------------------------------|
| Resource Type              | List                        | Map                              |
| Resource Identifier        | Index (`[0]`, `[1]`, `[2]`) | Key (value from map or set)      |
| Sensitive to Order Changes | ✅ Yes                       | ❌ No                             |
| Ideal Use Case             | Fixed-size lists            | Dynamic or keyed data structures |

## Example Output Comparison

Using `count`:

```
local_file.pet[0]
local_file.pet[1]
local_file.pet[2]
```

Using `for_each`:

```
local_file.pet["/root/sdcard.txt"]
local_file.pet["/root/dogs.txt"]
local_file.pet["/root/cats.txt"]
```

## 4. Additional Meta Arguments

Other useful meta arguments in Terraform include:

- `provider` – Specify which provider instance to use.
- `provisioner` – Run scripts or commands after resource creation.
- `backend` – Define where Terraform stores its state (e.g., S3, local).

These will be covered later in the course.

## Summary

| Meta Argument    | Purpose                                                          | Key Benefit                                         |
|------------------|------------------------------------------------------------------|-----------------------------------------------------|
| **`count`**      | Create multiple instances of a resource based on a numeric value | Simple looping mechanism                            |
| **`for_each`**   | Create multiple instances identified by unique keys              | Avoids unwanted replacements                        |
| **`depends_on`** | Define explicit dependencies                                     | Control resource creation order                     |
| **`lifecycle`**  | Manage create/update/destroy behavior                            | Prevent accidental deletions or control update flow |

Key Takeaways

- Use `count` for simple, fixed loops.
- Use `for_each` when working with maps, sets, or dynamic lists.
- `for_each` is safer and helps avoid configuration drift caused by index-based replacements.