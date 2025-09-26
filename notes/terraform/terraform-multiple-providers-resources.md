# Using Multiple Providers and Resources in Terraform

## 1. Multiple Providers in a Single Configuration

- Until now, we used a single provider: **local** (to create a file).
- Terraform supports **multiple providers** within the same configuration.
- Example of adding a second provider: **random**

## 2. Random Provider

- **Purpose:** Generates random values (IDs, integers, passwords, pet names, etc.)
- **Example Resource:** `random_pet`
  - Generates a random pet name.

### Resource Block Example

```hcl
resource "random_pet" "my_ dpet" {
  prefix    = "dog"
  separator = "-"
  length    = 2
}
```

**Breakdown**:

- `random_pet` → resource type
  - `random` → provider
  - `pet` → resource type
- `my_pet` → logical name for this resource.
- **Arguments**:
  - `prefix` → string to prepend to the name.
  - `separator` → character(s) separating prefix and generated name.
  - `length` → number of words in the generated name.

## 3. Combined Configuration Example

- Now our `main.tf` contains:
  - Local file resource (from previous lecture).
  - Random pet resource (newly added).

## 4. Re-Initialization

- When adding a new provider, you must re-run:

```bash
terraform init
```

- Downloads and installs the plugin for the new provider (`random`).
- Reuses previously installed providers (`local`).

## 5. Execution & Apply

- **Run**:

```bash
terraform plan
```

- **Shows that**:
  - Existing `local_file` resource remains unchanged.
  - New `random_pet` resource will be created.
- **Apply configuration**:

```bash
terraform apply
```

- Creates the `random_pet` resource.
- Displays an attribute called `id` containing the generated pet name.

## 6. Notes & Takeaways

- Multiple providers can coexist in a single configuration directory.
- Always run `terraform init `after adding a new provider.
- The `random` provider is a logical provider — it does not provision real infrastructure.
- The output (`id` attribute) is displayed on screen after apply.
- Random pet names may represent any pet — not limited to dogs (dog icon used as a visual metaphor).
