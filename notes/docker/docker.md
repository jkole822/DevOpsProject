# 🐳 Docker Notes

## Why Docker?

- Running complex application stacks (Node.js, MongoDB, Redis, Ansible, etc.) led to:
  - Compatibility issues between services, libraries, and OS.
  - The “matrix from hell”: different versions of components not working well together. 
  - Onboarding challenges: new developers needed hundreds of manual setup steps. 
  - Environment drift: dev/test/prod not consistent → “works on my machine” problem.
- Docker solved this by letting each component run in its own container, with its own dependencies, isolated from others but sharing the host OS kernel.

## What Are Containers?

- Isolated environments (own processes, services, network interfaces, file mounts). 
- Share the same OS kernel of the host. 
- Not new — Docker builds on earlier container tech (LXC, etc.). 
- Docker makes containers easy to use for developers.

## OS Kernel Basics

- Linux OS = kernel (interacts with hardware) + software (drivers, UI, dev tools, etc.). 
- Containers share the host kernel, but include only the extra software they need. 
- Linux-only: You cannot run Windows containers on a Linux host.
  - Running Linux containers on Windows actually runs them inside a hidden Linux VM.

## Containers vs Virtual Machines
| Feature	      | Containers | 	Virtual Machines        |
|---------------| --- |--------------------------|
| Isolation	    | Share kernel, less isolated | Fully isolated (own OS)  |
| Size	         | Lightweight (MBs)	| Heavy (GBs)              |
| Startup time	 | Seconds	| Minutes                  |
| OS required	  | Only app dependencies	| Full OS per VM           |
| Use cases	    | Packaging & shipping apps	| Running multiple OS types |

👉 It’s not containers vs VMs, but containers + VMs:
- Containers run inside VMs in large-scale environments.
- Use VMs for infrastructure flexibility, containers for app portability.
- A hypervisor is software (or firmware) that lets you run multiple virtual machines (VMs) on a single physical machine. There are two main types:
  - Type 1 (bare-metal hypervisor): Runs directly on the physical hardware.
    - Examples: VMware ESXi, Microsoft Hyper-V, Xen.
    - Think of it as replacing the host operating system.
  - Type 2 (hosted hypervisor): Runs on top of an existing operating system.
    - Examples: VirtualBox, VMware Workstation.
    - Here, the hypervisor is just another program you install on your computer.
- 👉 Each VM under a hypervisor emulates a full machine: its own OS kernel, drivers, and libraries. That’s why VMs are heavier than containers.
- In contrast, containers don’t need a hypervisor. They run directly on the host OS kernel, but keep separate user space environments. That’s why they’re so much lighter — they don’t emulate a full OS, just package the app and its dependencies.

## Docker Images & Containers

- Image = template (like a VM template).
- Container = running instance of an image.
- Many official/public images exist on Docker Hub (OSes, DBs, services, etc.).
- Workflow:
  1. Developers + Ops create a Dockerfile (requirements + config). 
  2. Build an image from it. 
  3. Run containers from that image on any host with Docker installed. 
  4. Works consistently across dev/test/prod.

## DevOps Angle

- Traditional flow: Devs build → Ops deploy (lots of miscommunication).
- With Docker:
  - Devs + Ops define environment together in a Dockerfile. 
  - Guarantees apps run the same everywhere. 
  - Supports CI/CD pipelines and modern DevOps practices.

✅ Key takeaway: Docker simplifies compatibility, environment setup, scaling, and deployment by packaging applications into lightweight, portable containers.