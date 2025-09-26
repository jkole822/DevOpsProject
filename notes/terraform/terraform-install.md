# Terraform Installation & Basics

## 1. Installing Terraform

- **Download:**  
  Terraform can be downloaded as a single binary/executable file from the [Terraform download page](https://www.terraform.io/downloads.html).
- **Installation:**
  - Simply download the file and copy it to your system path.
  - Supported on **Windows**, **macOS**, and several **Linux distributions**.
- **Verify Installation:**  
  Run the command:

```bash
terraform version
```

- Version Used in Course:
  - Latest version at the time of recording: 0.13
  - All course examples use Terraform 0.13 on Linux.

## 2. Starting with Terraform

- Terraform uses configuration files written in HCL (HashiCorp Configuration Language, not SQL) to define infrastructure.
- These files:
  - Have a `.tf` extension.
  - Can be created using:
    - Simple text editors (Notepad, Notepad++).
    - CLI editors (Vim, Nano).
    - Any IDE of your choice.

## 3. What is a Resource?

- **Definition:**
  A resource is an object managed by Terraform.
- **Examples of Resources:**
  - Local file on a machine.
  - Virtual machines on the cloud (e.g., AWS EC2 instances).
  - Cloud services:
    - S3 Buckets
    - DynamoDB Tables
    - IAM Users, Groups, Roles, Policies
  - Resources on major cloud providers:
    - **GCP**: Compute Engine, App Engine
    - **Azure**: Databases, Active Directory
  - Many more (hundreds of possible resources across cloud & on-prem infrastructure).

## 4. Resources Used in Early Sections

- To simplify learning, the course focuses on:
  1. Local file resource type.
  2. Random provider resource type (special kind of resource).
- Purpose:
  - Understand Terraform basics:
    - Resource lifecycle
    - HCL syntax
    - Basic workflow
  - Build a strong foundation before moving on to real-life infrastructure examples.

## 5. Key Takeaways

- Terraform installation is simple: download → copy to path → verify.
- Resources are the core building blocks of Terraform.
- Start with simple resource types to understand concepts before provisioning real infrastructure.
- Knowledge gained can be applied to any cloud provider or on-premise setup.
