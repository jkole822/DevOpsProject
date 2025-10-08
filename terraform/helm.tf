resource "helm_release" "ebs_csi_driver" {
  name       = "aws-ebs-csi-driver"
  namespace  = "kube-system"
  repository = "https://kubernetes-sigs.github.io/aws-ebs-csi-driver"
  chart      = "aws-ebs-csi-driver"
  version    = "2.23.0"

  set = [
    {
      name  = "controller.serviceAccount.create"
      value = "false"
    },
    {
      name  = "controller.serviceAccount.name"
      value = kubernetes_service_account.ebs_csi_driver.metadata[0].name
    }
  ]

  depends_on = [
    module.eks
  ]
}

resource "helm_release" "aws_lb_controller" {
  name       = "aws-load-balancer-controller"
  namespace  = "kube-system"
  repository = "https://aws.github.io/eks-charts"
  chart      = "aws-load-balancer-controller"
  version    = "1.10.0"

  set = [
    {
      name  = "clusterName"
      value = module.eks.cluster_name
    },
    {
      name  = "vpcId"
      value = module.vpc.vpc_id
    },
    {
      name = "serviceAccount.create",
      value = "false"
    },
    {
      name = "serviceAccount.name",
      value = kubernetes_service_account.alb_controller.metadata[0].name
    },
    {
      name  = "region"
      value = var.aws_region
    }
  ]

  depends_on = [
    module.eks,
    kubernetes_service_account.alb_controller
  ]
}

resource "helm_release" "umbrella" {
  name         = "dev-ops-project"
  namespace    = "default"
  chart        = "../helm/umbrella"
  force_update = true

  values = [
    file("../helm/umbrella/values.yaml") # optional
  ]

  depends_on = [
    helm_release.aws_lb_controller,
    helm_release.ebs_csi_driver,
    kubernetes_secret.auth_secret,
    kubernetes_secret.task_secret,
    kubernetes_secret.worker_secret
  ]
}


