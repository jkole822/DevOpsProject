module "vpc" {
  source = "terraform-aws-modules/vpc/aws"

  name = "my-vpc"
  cidr = "10.0.0.0/16"

  azs             = ["us-east-1a", "us-east-1b", "us-east-1c"]
  private_subnets = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
  public_subnets  = ["10.0.101.0/24", "10.0.102.0/24", "10.0.103.0/24"]

  enable_nat_gateway = true
  enable_vpn_gateway = true

  tags = {
    Terraform   = "true"
    Environment = "dev"
  }
}

module "eks" {
  source  = "terraform-aws-modules/eks/aws"
  version = "~> 21.0"

  name               = "my-cluster"
  kubernetes_version = "1.32"

  endpoint_public_access                   = true
  enable_cluster_creator_admin_permissions = true

  vpc_id     = module.vpc.vpc_id
  subnet_ids = module.vpc.private_subnets

  addons = {
    coredns = {}
    eks-pod-identity-agent = {
      before_compute = true
    }
    kube-proxy = {}
    vpc-cni = {
      before_compute = true
    }
  }

  eks_managed_node_groups = {
    eks_nodes_a = {
      ami_type       = "AL2_x86_64"
      instance_types = ["t3.medium"]
      subnet_ids     = [module.vpc.private_subnets[0]] # us-east-1a
      desired_size   = 1
      max_size       = 1
      min_size       = 1
    }

    eks_nodes_b = {
      ami_type       = "AL2_x86_64"
      instance_types = ["t3.medium"]
      subnet_ids     = [module.vpc.private_subnets[1]] # us-east-1b
      desired_size   = 1
      max_size       = 1
      min_size       = 1
    }

    eks_nodes_c = {
      ami_type       = "AL2_x86_64"
      instance_types = ["t3.medium"]
      subnet_ids     = [module.vpc.private_subnets[2]] # us-east-1c
      desired_size   = 1
      max_size       = 1
      min_size       = 1
    }
  }

  tags = {
    Terraform   = "true"
    Environment = "dev"
  }
}

