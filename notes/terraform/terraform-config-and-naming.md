# Terraform Configuration Directory & File Naming Conventions

## 1. Configuration Directory

- **Definition:**  
  The directory where `.tf` files (Terraform configuration files) are stored.
- Example:

```
/root/terraform-local-file/
```

- Contains `local.tf` (our first configuration file).
- Terraform treats this directory as the **working directory**.

## 2. Multiple Configuration Files

- A configuration directory **can contain multiple `.tf` files**.
- Example:
- `local.tf` → creates `/root/pet.txt`
- `cat.tf` → creates `/root/cat.txt`
- Terraform **automatically loads and processes all `.tf` files** in the directory when you run commands like:

```bash
terraform plan
terraform apply
```

## 3. Common Practice: Single Main File

- Instead of multiple files, many projects use a single configuration file containing all resource blocks.
- This file is commonly named:

```
main.tf
```

## 4. Other Common Configuration Files

Terraform projects often include additional `.tf` files for better organization:

- `variables.tf` – defines input variables.
- `outputs.tf` – defines output values.
- `providers.tf` – specifies provider configuration.

## 5. Key Takeaways

- Terraform loads all `.tf` files in the working directory.
- You can split configurations across multiple files or keep them in a single main.tf.
- Using separate files (`variables.tf`, `outputs.tf`, `providers.tf`) helps organize and scale larger projects.
