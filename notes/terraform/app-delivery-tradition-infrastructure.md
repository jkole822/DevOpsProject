# Application Delivery: Traditional Infrastructure → Cloud → Infrastructure as Code

## 1. Traditional Infrastructure Model

### Workflow

1. **Business Requirements**

   - Business defines requirements for a new application.
   - Business analyst gathers and converts them into high-level technical requirements.

2. **Architecture & Design**

   - Solution Architect designs the system architecture:
     - Specifies type, spec, and number of servers
     - Considers front-end servers, back-end servers, databases, load balancers, etc.

3. **Provisioning Infrastructure**

   - Deployed in **on-premise data centers**.
   - If extra hardware is needed:
     - Procurement team orders hardware from vendors.
     - Delivery may take **days → weeks → months**.

4. **Deployment Steps**
   - Field engineers rack and stack equipment.
   - System administrators configure servers.
   - Network admins configure network access.
   - Storage admins allocate storage.
   - Backup admins configure backup policies.
   - Finally, application teams deploy their applications.

### Disadvantages

- **Long lead time:** Weeks or months just to get infrastructure ready.
- **Manual hand-offs:** Multiple teams → high coordination overhead.
- **Scaling is slow:** Hard to scale up/down on demand.
- **High cost:** Hardware + operations + human resources.
- **High risk of human error:** Many manual steps → inconsistent environments.
- **Underutilized resources:** Servers sized for peak load → wasted capacity during off-peak.

## 2. Virtualization & Cloud Adoption

### Advantages of Moving to the Cloud

- **Faster provisioning:** Spin up VMs in **minutes**, not months.
- **Reduced costs:** No need to buy/manage physical hardware or data centers.
- **Managed services:** Hardware, data center, and core infrastructure are cloud provider’s responsibility.
- **APIs available:** Enables automation and programmatic control.
- **Auto-scaling:** Elastic infrastructure reduces resource waste.
- **Faster time-to-market:** Delivery cycle reduced significantly.

### Limitations

- Provisioning through **management console** is not scalable for large environments.
- Process overhead and team hand-offs still remain.
- Risk of human error persists → inconsistent environments.

## 3. Emergence of Automation & Infrastructure as Code (IaC)

### Early Solutions

- Organizations started writing **custom scripts**:
  - Shell scripts
  - Python, Ruby, Perl, PowerShell

### Goals

- Automate infrastructure provisioning
- Reduce delivery time
- Ensure consistent, repeatable environments
- Leverage cloud provider APIs for automation

### Outcome

- These efforts evolved into a category of tools and practices now called **Infrastructure as Code (IaC)**:
  - Allows infrastructure to be defined, versioned, and deployed like application code.
  - Improves reliability, repeatability, and scalability of deployments.
