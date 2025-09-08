# Kubernetes Setup Options & Minikube

## Ways to Set Up a Kubernetes Cluster

Local setups (for developers/learning):

- Minikube
- Microk8s

Production-grade setup:

- kubeadm: bootstraps and manages full production clusters

Cloud-hosted solutions:

- GCP, AWS, Azure, IBM Cloud, and many others

👉 Course provides a real Kubernetes cluster in-browser for hands-on practice (no setup needed).

## Minikube Overview

- Bundles all Kubernetes components into a single pre-configured image
- Creates a single-node Kubernetes cluster for local testing/learning
- Distributed as an ISO image that Minikube downloads automatically
- Runs inside a virtualization platform:
  - Oracle VirtualBox, VMware Fusion (cross-platform)
  - Hyper-V (Windows)
  - KVM (Linux)
- Requirements to Run Minikube
  - A hypervisor installed
  - kubectl (Kubernetes CLI) installed
  - Minikube executable installed

✅ Key Takeaway:
Minikube is the fastest way to spin up a local Kubernetes cluster for practice, but in this course you can also rely entirely on the provided in-browser Kubernetes lab environment.
