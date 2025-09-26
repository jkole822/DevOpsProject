# Introduction to Infrastructure as Code (IaC)

## 1. What is Infrastructure as Code (IaC)?

- **Definition:**  
  The practice of codifying the entire infrastructure provisioning process so it can be:

  - Defined
  - Provisioned
  - Configured
  - Updated
  - Destroyed  
    …using code instead of manual processes.

- **Scope:**  
  Nearly any infrastructure component can be managed as code:

  - Servers
  - Databases
  - Networks
  - Storage
  - Application configuration

- **Benefits:**
  - Automates and accelerates provisioning
  - Improves consistency
  - Reduces human error
  - Enables version control and collaboration

## 2. From Shell Scripts to IaC Tools

- Early automation was done via **shell scripts**.
- Problems with shell scripts:

  - Require programming expertise
  - Hard to maintain
  - Not easily reusable
  - Complex logic

- **IaC Tools Solve These Issues:**
  - Human-readable, easy-to-learn syntax
  - Maintainable and reusable
  - Example tools:
    - **Terraform:** Defines infrastructure using declarative configuration
    - **Ansible:** Automates configuration and provisioning via playbooks

## 3. Categories of IaC Tools

IaC tools can be broadly classified into three categories:

### 3.1 Configuration Management Tools

- **Examples:** Ansible, Chef, Puppet, SaltStack
- **Purpose:** Install, configure, and manage software on **existing infrastructure**.
- **Features:**
  - Consistent code structure
  - Can target multiple remote resources simultaneously
  - **Version-controllable** (playbooks stored in Git, etc.)
  - **Idempotent:** Running the same code multiple times only applies necessary changes.

### 3.2 Server Templating Tools

- **Examples:** Docker, Vagrant, Packer
- **Purpose:** Create custom **machine or container images** that already include all required software and dependencies.
- **Benefits:**

  - Reduces post-deployment installation/configuration steps
  - Promotes **immutable infrastructure**:
    - Once deployed, servers/containers are not changed.
    - Updates are made by **rebuilding the image** and redeploying.

- **Common Examples of Pre-Built Images:**
  - VM images from OSBoxes.org
  - Amazon Machine Images (AMIs) on AWS
  - Docker images on Docker Hub

### 3.3 Infrastructure Provisioning Tools

- **Examples:** Terraform, AWS CloudFormation
- **Purpose:** Provision infrastructure components using **declarative code**:
  - VMs, databases, networks (VPCs, subnets), security groups, storage, etc.
- **Key Differences:**
  - **CloudFormation:** AWS-specific
  - **Terraform:** Vendor-agnostic (supports multiple cloud providers through plugins)

## 4. Key Takeaways

- **IaC replaces manual provisioning with code**, making infrastructure management faster and more reliable.
- **Configuration management** handles software setup.
- **Server templating** focuses on immutable, pre-configured images.
- **Provisioning tools** create the infrastructure itself, often across multiple clouds.
