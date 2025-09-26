# Introduction to Terraform

## 1. What is Terraform?

- **Definition:**  
  Terraform is a popular **Infrastructure as Code (IaC) provisioning tool** developed by HashiCorp.
- **Key Features:**
  - Free and open-source
  - Installs as a **single binary**
  - Allows you to **build, manage, and destroy infrastructure in minutes**
  - Supports **multi-cloud and on-premise platforms**:
    - Public cloud: AWS, GCP, Azure
    - Private/on-prem: vSphere, physical servers
    - Networking, monitoring, databases, and version control platforms

## 2. How Terraform Works

### 2.1 Providers

- Providers allow Terraform to **interact with third-party platforms via APIs**
- Examples:
  - Cloud: AWS, Azure, GCP
  - Network: BigIP, Cloudflare DNS, Palo Alto
  - Monitoring: Datadog, Wavefront, Sumo Logic
  - Databases: MongoDB, MySQL, PostgreSQL, InfluxDB
  - Version Control: GitHub, Bitbucket, GitLab
- **Hundreds of providers** are supported, making Terraform highly versatile.

### 2.2 Configuration Language (HCL)

- Terraform uses **HCL (HashiCorp Configuration Language)**:
  - Declarative language to define infrastructure as code
  - Infrastructure resources are defined as **blocks in `.tf` files**
  - Syntax is **human-readable** and beginner-friendly
- Example: Provisioning a new EC2 instance on AWS
- Can be stored in **version control** for collaboration and reusability

### 2.3 Declarative Nature

- **Declarative:** You specify the **desired state** of your infrastructure
- Terraform determines the steps to go from the **current state** to the **desired state**
- Ensures consistent infrastructure without manually coding step-by-step instructions

## 3. Terraform Workflow

Terraform works in **three main phases**:

1. **Init**
   - Initializes the project
   - Identifies the providers to use
2. **Plan**
   - Drafts a plan to achieve the desired state
   - Shows changes Terraform will make
3. **Apply**
   - Executes the plan
   - Modifies the infrastructure to reach the desired state
   - Can **correct drift** in infrastructure on subsequent runs

## 4. Key Concepts

### 4.1 Resources

- Every object Terraform manages is called a **resource**
- Examples: compute instances, databases, storage
- Terraform manages the **entire lifecycle**:
  - Provisioning → Configuration → Decommissioning

### 4.2 State Management

- Terraform keeps track of the **current state of infrastructure**
- Ensures resources match the **desired state** at all times
- The state acts as a **blueprint of deployed infrastructure**

### 4.3 Data Sources

- Terraform can **read attributes of existing resources**
- Data can be used to **configure other resources** in Terraform

### 4.4 Importing Existing Resources

- Terraform can **import resources created outside of Terraform**
- Brings them under Terraform management for consistent future updates

## 5. Enterprise Features

- **Terraform Cloud/Enterprise** provides:
  - Simplified team collaboration
  - Centralized UI for managing deployments
  - Improved security
  - Centralized infrastructure management
- These features make Terraform **enterprise-ready** for large organizations
