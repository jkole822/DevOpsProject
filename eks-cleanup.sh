#!/bin/bash

# --- CONFIGURE THESE VARIABLES ---
CLUSTER_NAME="devops-project"
REGION="us-east-1"
# --------------------------------

echo "🔎 Checking if cluster '$CLUSTER_NAME' exists in $REGION..."
if ! aws eks describe-cluster --region "$REGION" --name "$CLUSTER_NAME" >/dev/null 2>&1; then
  echo "❌ No cluster named '$CLUSTER_NAME' found in $REGION. Nothing to delete."
  exit 0
fi

echo "🧹 Deleting EKS cluster: $CLUSTER_NAME ..."
eksctl delete cluster --name "$CLUSTER_NAME" --region "$REGION"

echo "🧹 Cleaning up leftover Load Balancers..."
LB_ARNS=$(aws elbv2 describe-load-balancers --region "$REGION" --query "LoadBalancers[].LoadBalancerArn" --output text)
for LB in $LB_ARNS; do
  NAME=$(aws elbv2 describe-load-balancers --load-balancer-arns "$LB" --region "$REGION" --query "LoadBalancers[0].LoadBalancerName" --output text)
  if [[ $NAME == *"$CLUSTER_NAME"* ]]; then
    echo "   ➤ Deleting LB: $NAME"
    aws elbv2 delete-load-balancer --load-balancer-arn "$LB" --region "$REGION"
  fi
done

echo "🧹 Cleaning up orphaned EBS volumes..."
VOLUMES=$(aws ec2 describe-volumes --region "$REGION" --query "Volumes[?State=='available'].VolumeId" --output text)
for VOL in $VOLUMES; do
  echo "   ➤ Deleting volume: $VOL"
  aws ec2 delete-volume --volume-id "$VOL" --region "$REGION"
done

echo "✅ Cleanup complete!"
