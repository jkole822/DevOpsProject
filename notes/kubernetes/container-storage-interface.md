# CSI

CSI stands for Container Storage Interface.

It’s a standardized interface that allows container orchestration systems (like Kubernetes) to provision, attach, mount, and manage storage volumes from different storage providers in a consistent way.

**Key Points:**
- Abstracts storage providers
  - CSI allows Kubernetes to work with multiple storage backends (EBS, EFS, GCP Persistent Disks, etc.) without needing custom code for each one. 
- Dynamic provisioning 
  - With CSI drivers, Kubernetes can dynamically create storage volumes when a **PersistentVolumeClaim** (PVC) is requested. 
- Decouples orchestration and storage
  - Storage vendors provide CSI drivers, which handle all the specifics of volume creation, attachment, and deletion. Kubernetes just talks to the driver using the standard CSI API.

--- 

In your case:
- EBS CSI Driver is the driver that implements the CSI spec for Amazon EBS volumes, letting Kubernetes dynamically create and attach EBS volumes to pods.
- Need to include permissions (refer to `aws.tf`)