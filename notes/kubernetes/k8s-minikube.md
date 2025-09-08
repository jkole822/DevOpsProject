# Installing a Basic Kubernetes Cluster with Minikube

## Approach

- Using Minikube as the lab solution (beginner-friendly)
- Focus on basic operations only
- kubeadm and other advanced setups are explored later in the course

## Step 1: Install kubectl (Kubernetes CLI)

- Required before Minikube (so Minikube can auto-configure it)
- Manages Kubernetes resources and clusters (local or remote)
- Installation (Linux example):

```bash
curl -LO "https://storage.googleapis.com/kubernetes-release/release/v1.18.0/bin/linux/amd64/kubectl"
chmod +x kubectl
sudo mv kubectl /usr/local/bin/
kubectl version --client
```

- Other installation methods:
  - Package managers (apt, brew, chocolatey, etc.)
  - Check official docs for Windows/macOS/Linux instructions

## Step 2: Check Virtualization Support

- Must be enabled for Minikube to run a VM
- Linux example:

```bash
grep -E --color 'vmx|svm' /proc/cpuinfo
```

- If not enabled → enable in BIOS (check laptop manual or manufacturer guide)

## Step 3: Install a Hypervisor

- Options: VirtualBox, KVM (Linux), Hyper-V (Windows)
- VirtualBox chosen in demo
  - Download installer from VirtualBox website
  - Works on Linux, Windows, macOS
- Alternative: Use Docker driver (no VM), but may cause security/data loss issues → not recommended

## Step 4: Install Minikube

- Download latest binary and install:

```bash
curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64
chmod +x minikube-linux-amd64
sudo mv minikube-linux-amd64 /usr/local/bin/minikube
```

- Verify:

```bash
minikube version
```

## Step 5: Start Minikube Cluster

- Start with VirtualBox driver:

```bash
minikube start --driver=virtualbox
```

- Process:
  - Downloads Minikube ISO
  - Creates VM in VirtualBox (2 CPUs, 2 GB RAM by default)
  - Installs Kubernetes binaries
- Verify VM is running in VirtualBox UI

## Step 6: Verify Cluster Setup

- Check status:

```bash
minikube status
```

- Ensure:

  - Control plane
  - kubelet
  - API server
  - Config → all running

- Check nodes:

```bash
kubectl get nodes
```

- Output: single-node cluster (minikube), status Ready

## Step 7: Deploy and Test an App

1. Create Deployment:

```bash
kubectl create deployment hello-minikube --image=k8s.gcr.io/echoserver:1.4
kubectl get deployments
```

2. Expose Deployment as a Service:

```bash
kubectl expose deployment hello-minikube --type=NodePort --port=8080
```

3. Get Service URL:

```bash
minikube service hello-minikube --url
```

4. Open URL in browser → see basic app response

## Step 8: Clean Up

1. Delete service:

```bash
kubectl delete service hello-minikube
```

2. Delete deployment:

```bash
kubectl delete deployment hello-minikube
```

Deployment may show Terminating for a few seconds before removal

---

✅ Result: A working single-node Kubernetes cluster running on Minikube with VirtualBox.
