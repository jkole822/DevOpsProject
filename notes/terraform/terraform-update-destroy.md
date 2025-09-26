# Terraform Destroy Command

## 1. Updating a Resource

- When a change is applied (e.g., new permissions):
  - Terraform **deletes the existing file**.
  - **Recreates it** with the updated configuration.

## 2. Deleting Infrastructure

- Use the **`terraform destroy`** command to remove all resources defined in the current configuration.

### Workflow:

1. Run:

```bash
terraform destroy
```

2. Terraform displays an execution plan:
   - Resources marked with a − (minus symbol) indicate they will be destroyed.
3. Confirm by typing:

```bash
yes
```

4. Terraform proceeds to delete the resources.

## 3. Example

- Configuration creates `/root/pet.txt`.
- Running `terraform destroy`:
  - Shows the resource with a minus symbol.
  - Deletes `/root/pet.txt` after confirmation.

## 4. Key Takeaways

- Destroy = Clean-up → removes all resources from the current working directory’s state.
- Always review the plan carefully before confirming.
- Useful for cleaning up test or temporary infrastructure.
