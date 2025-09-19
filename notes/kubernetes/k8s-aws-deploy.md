# 🚀 Steps to Deploy Kubernetes Workloads on AWS

## 1. Provision a Kubernetes Cluster on AWS

You have a few options:

- EKS (Elastic Kubernetes Service) → fully managed, AWS handles the control plane.
- kOps → popular for rolling your own cluster on EC2.
- Self-managed on EC2 → less common now (lots of overhead).

👉 Most teams go with EKS.

You can create an EKS cluster with:

- eksctl (simple CLI tool, recommended):

```bash
eksctl create cluster \
  --name my-cluster \
  --version 1.30 \
  --region us-east-1 \
  --nodegroup-name my-nodes \
  --node-type t3.medium \
  --nodes 2 \
  --nodes-min 2 \
  --nodes-max 4 \
  --managed
```
- `--version 1.30` in the eksctl create cluster command refers to the Kubernetes version your EKS cluster will run.
  - EKS supports multiple Kubernetes versions — AWS usually supports the latest 4–5 minor versions at any given time.
  - 1.30 is currently one of the newest stable Kubernetes releases supported by EKS (as of mid-2025).
  - Specifying --version 1.30 tells eksctl to provision your control plane and worker nodes with Kubernetes v1.30.
  - If you omit --version, eksctl will use the default version supported by EKS (usually the latest stable one).
- After creation, verify nodes:
```bash
kubectl get nodes
```

- **Terraform** or **AWS CDK** if you want Infrastructure as Code.

## 2. Configure kubectl to talk to AWS cluster

Once the cluster is up, update your kubeconfig:

```bash
aws eks update-kubeconfig --region us-east-1 --name my-cluster
```

Verify connection:
```bash
kubectl get svc
kubectl get nodes
```

Now `kubectl get nodes` should show AWS worker nodes instead of your local machine.

## 3. Push Your Docker Images to AWS ECR

AWS won’t pull images from your local machine, so you need to publish them:

```bash
# Authenticate Docker with ECR
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin <account_id>.dkr.ecr.us-east-1.amazonaws.com

# Create a repo (if not exists)
aws ecr create-repository --repository-name my-app

# Tag and push your image
docker tag my-app:latest <account_id>.dkr.ecr.us-east-1.amazonaws.com/my-app:latest
docker push <account_id>.dkr.ecr.us-east-1.amazonaws.com/my-app:latest
```

## 4. Apply Your Kubernetes Manifests

Now your manifests (deployment.yaml, service.yaml, etc.) need to reference the ECR image:

```yaml
containers:
  - name: my-app
    image: <account_id>.dkr.ecr.us-east-1.amazonaws.com/my-app:latest
```

Deploy with:

```bash
kubectl apply -f k8s/
```

### Using Helm:
1. Install Helm
```bash
curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash
helm version
```

2. Create a Helm Chart
```bash
helm create my-app
```

3. Deploy the Chart to EKS
```bash
helm install my-app ./my-app
```
- my-app is the release name.
- ./my-app points to your chart directory.
- Check deployment:
```bash
kubectl get pods
kubectl get svc
```

- If the service is a LoadBalancer, AWS ELB is automatically provisioned.

4. Update Your Deployment

- If you change the image or configuration in values.yaml:
```bash
helm upgrade my-app ./my-app
```

- Helm automatically applies changes without destroying the entire deployment.

5. Uninstall Your App
```bash
helm uninstall my-app
```
- This removes all resources created by the Helm release.

## 5. Expose Your Application

On AWS, you typically use a LoadBalancer Service:

```yaml
apiVersion: v1
kind: Service
metadata:
  name: my-service
spec:
  type: LoadBalancer
  selector:
    app: my-app
  ports:
    - port: 80
      targetPort: 3000
```

That tells AWS to spin up an ELB (Elastic Load Balancer). Run:

```bash
kubectl get svc my-service
```

You’ll see an external IP (AWS load balancer DNS name).

### OR Set up Ingress
- Install AWS ALB Ingress Controller:

```bash
kubectl apply -k "github.com/aws/eks-charts/stable/aws-load-balancer-controller//crds?ref=master"
```

- Deploy the AWS Load Balancer Controller either via Helm or manifest.

```bash
# Add the Helm repo
helm repo add eks https://aws.github.io/eks-charts
helm repo update

# Install controller
helm install aws-load-balancer-controller eks/aws-load-balancer-controller \
  -n kube-system \
  --set clusterName=<CLUSTER-NAME> \
  --set serviceAccount.create=true \
  --set region=<REGION> \
  --set vpcId=<YOUR-VPC-ID> \
  --set image.repository=602401143452.dkr.ecr.us-east-1.amazonaws.com/amazon/aws-load-balancer-controller
```
- Make sure your cluster has the IAM role permissions for ALB management (IAM OIDC + AWSLoadBalancerController IAM policy).
- Your existing Ingress YAML should have the correct annotations so the controller knows to create an ALB:

```bash
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: my-app-ingress
  annotations:
    kubernetes.io/ingress.class: alb
    alb.ingress.kubernetes.io/scheme: internet-facing
spec:
  rules:
    - http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: my-app-service
                port:
                  number: 80
```

- Once both the controller is running and your Ingress is applied:
  - The controller watches the Ingress resource
  - Automatically provisions an ALB in AWS
  - Creates listeners, target groups, security group rules, etc.

## 6. (Optional) Add DNS & HTTPS

- Use **Route 53** to map a domain name.
- Use **cert-manager** to auto-provision SSL certificates via Let’s Encrypt.

# 🔄 Switching Between Local & AWS Kubernetes

## 1. See all available contexts

```bash
kubectl config get-contexts
```

Example output:

| CURRENT  | NAME            | CLUSTER         | AUTHINFO        | NAMESPACE |
|----------| --------------- | --------------- |-----------------|-----------|
| *        | minikube        | minikube        | minikube        | default   |
|          | arn:aws:eks:... | arn:aws:eks:... | arn:aws:eks:... | default   |

- `*` marks the current context (active cluster).
- One is your local cluster (minikube, kind, or docker-desktop).
- One is AWS EKS.

## 2. Switch context

```bash
kubectl config use-context minikube
```

(or `kind-kind`, `docker-desktop`, depending on your local setup)

To switch back to AWS:

```bash
kubectl config use-context arn:aws:eks:us-east-1:123456789:cluster/my-cluster
```

## 3. Verify

```bash
kubectl get nodes
```

If local: you’ll see 1–3 nodes on your laptop.

If AWS: you’ll see EC2 instances.

---

💡 Tip: You can make life easier by renaming contexts:

```bash
kubectl config rename-context arn:aws:eks:us-east-1:123456789:cluster/my-cluster aws-eks
```

Then just use:

```bash
kubectl config use-context aws-eks
kubectl config use-context minikube
```
