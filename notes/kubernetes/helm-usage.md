Run `minikube start`

If your previous cluster had addons like ingress, metrics-server, or dashboard:

```bash
minikube addons enable ingress
minikube addons enable metrics-server
minikube addons enable dashboard
```

Create secrets if they don't exist:

```bash
kubectl create secret generic app-secrets --from-env-file=.env
```

Run this within the umbrella directory:

```bash
helm dependency update .
helm upgrade --install my-release .
```

Use port-forwarding for the APIs:

```bash
kubectl port-forward svc/<CLUSTER_IP_NAME> <LOCAL_PORT>:<CLUSTER_IP_PORT>
```

Use Minikube driver networking for the frontend:

```bash
minikube service frontend-node-port  --url
```

When done:

```bash
minikube stop
```

When wanting to delete:

```bash
minikube stop
minikube delete
```

When deploying on EKS, make sure ALB ingress controller exists.

Your annotations specify:

```yaml
kubernetes.io/ingress.class: alb
alb.ingress.kubernetes.io/scheme: internet-facing
```

This requires the AWS Load Balancer Controller to be installed in your cluster. Without it, the ingress won’t create a
Load Balancer. You can install it via Helm:

```bash
helm repo add eks https://aws.github.io/eks-charts
helm install aws-load-balancer-controller eks/aws-load-balancer-controller \
  -n kube-system \
  --set clusterName=my-cluster \
  --set serviceAccount.create=false \
  --set region=us-east-1 \
  --set vpcId=<your-vpc-id> \
  --set serviceAccount.name=aws-load-balancer-controller
```

But you can also use Terraform to create the ALB controller:

```hcl
resource "helm_release" "aws_lb_controller" {
  name       = "aws-load-balancer-controller"
  namespace  = "kube-system"
  repository = "https://aws.github.io/eks-charts"
  chart      = "aws-load-balancer-controller"
  version    = "1.10.8"

  depends_on = [module.eks] # ensure cluster exists

  set {
    name  = "clusterName"
    value = module.eks.cluster_name
  }
  set {
    name  = "vpcId"
    value = module.vpc.vpc_id
  }
  set {
    name  = "serviceAccount.create"
    value = "false"
  }
  set {
    name  = "region"
    value = var.aws_region
  }
  set {
    name  = "serviceAccount.name"
    value = "aws-load-balancer-controller"
  }
}
```

Troubleshooting EKS Resource Management
Need to establish kubeconfig from AWS CLI:

```bash
aws eks --region <your-region> update-kubeconfig --name <your-cluster-name>
```

This writes a kubeconfig to ~/.kube/config that kubectl can use.

Check node and pods

```bash
# list nodes
kubectl get nodes

# describe a node
kubectl describe node <node-name>

# list pods in kube-system (where CNI runs)
kubectl get pods -n kube-system

# describe the aws-node pod
kubectl describe pod <aws-node-pod-name> -n kube-system
```

Check the CNI

```bash
kubectl get daemonset aws-node -n kube-system
kubectl describe daemonset aws-node -n kube-system
```

- Each node should have a running aws-node pod.
- If pods are not running, check IAM role for the node group and subnet configuration.

If PVC are stuck in pending, run:

```bash
kubectl delete pods -n kube-system -l app.kubernetes.io/name=aws-ebs-csi-driver
```

Watch them restart and rebind PVCs:

```bash
kubectl get pods -n kube-system | grep ebs
kubectl get pvc
```

Need to include this for ALB Load Balancer Controller permissions (`refer to aws.tf)`:
```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "ec2:DescribeVpcs",
        "ec2:DescribeSubnets",
        "ec2:DescribeSecurityGroups",
        "ec2:DescribeInstances",
        "ec2:DescribeNetworkInterfaces",
        "ec2:CreateNetworkInterface",
        "ec2:AttachNetworkInterface",
        "ec2:DeleteNetworkInterface",
        "ec2:AssignPrivateIpAddresses",
        "ec2:UnassignPrivateIpAddresses",
        "elasticloadbalancing:CreateLoadBalancer",
        "elasticloadbalancing:DeleteLoadBalancer",
        "elasticloadbalancing:DescribeLoadBalancers",
        "elasticloadbalancing:DescribeTargetGroups",
        "elasticloadbalancing:CreateTargetGroup",
        "elasticloadbalancing:DeleteTargetGroup",
        "elasticloadbalancing:ModifyTargetGroup",
        "elasticloadbalancing:RegisterTargets",
        "elasticloadbalancing:DeregisterTargets",
        "elasticloadbalancing:DescribeListeners",
        "elasticloadbalancing:CreateListener",
        "elasticloadbalancing:DeleteListener",
        "elasticloadbalancing:ModifyListener",
        "elasticloadbalancing:DescribeRules",
        "elasticloadbalancing:CreateRule",
        "elasticloadbalancing:DeleteRule",
        "elasticloadbalancing:ModifyRule",
        "iam:ListServerCertificates",
        "iam:GetServerCertificate"
      ],
      "Resource": "*"
    }
  ]
}
```