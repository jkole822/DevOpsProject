# The annotation eks.amazonaws.com/role-arn links the service account to the IAM role.
# Make sure the namespace is kube-system, which is where the CSI driver is usually installed.
resource "kubernetes_service_account" "ebs_csi_driver" {
  metadata {
    name      = "ebs-csi-controller-sa"
    namespace = "kube-system"
    annotations = {
      "eks.amazonaws.com/role-arn" = aws_iam_role.ebs_csi_driver.arn
    }
  }
}

resource "kubernetes_service_account" "alb_controller" {
  metadata {
    name      = "aws-load-balancer-controller"
    namespace = "kube-system"
    annotations = {
      "eks.amazonaws.com/role-arn" = aws_iam_role.alb_controller.arn
    }
  }
}

resource "kubernetes_secret" "auth_secret" {
  metadata {
    name      = "auth-secret"
    namespace = "default"
  }

  data = {
    POSTGRES_DB       = var.auth_postgres_db
    POSTGRES_HOST     = var.auth_postgres_host
    POSTGRES_PORT     = var.auth_postgres_port
    POSTGRES_PASSWORD = var.auth_postgres_password
    POSTGRES_USER     = var.auth_postgres_user
    DATABASE_URL      = var.auth_database_url
    REDIS_URL         = var.redis_url
  }

  depends_on = [module.eks]
}

resource "kubernetes_secret" "task_secret" {
  metadata {
    name      = "task-secret"
    namespace = "default"
  }

  data = {
    POSTGRES_DB       = var.task_postgres_db
    POSTGRES_HOST     = var.task_postgres_host
    POSTGRES_PORT     = var.task_postgres_port
    POSTGRES_PASSWORD = var.task_postgres_password
    POSTGRES_USER     = var.task_postgres_user
    AUTH_URL          = var.auth_url
    DATABASE_URL      = var.task_database_url
    REDIS_URL         = var.redis_url
  }

  depends_on = [module.eks]
}

resource "kubernetes_secret" "worker_secret" {
  metadata {
    name      = "worker-secret"
    namespace = "default"
  }

  data = {
    DATABASE_URL = var.task_database_url
    REDIS_URL    = var.redis_url
  }

  depends_on = [module.eks]
}
