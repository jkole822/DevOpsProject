1) Create the folder layout

From your project root:

mkdir -p helm/charts
mkdir -p helm/umbrella
# (optional) create a spot for each service
mkdir -p helm/charts/auth-api
mkdir -p helm/charts/user-api


Suggested final layout:

helm/
umbrella/
Chart.yaml
values.yaml
charts/
auth-api/
Chart.yaml
values.yaml
templates/
deployment.yaml
service.yaml
ingress.yaml
user-api/
...

2) Make a subchart for each service (quick start)

For each microservice (example: auth-api):

cd helm/charts
helm create auth-api   # creates a starter chart structure
# remove the example templates you won't use and add your own
rm auth-api/templates/*.yaml  # or edit the defaults


Now move your existing k8s YAML (from k8s/) into helm/charts/auth-api/templates/ and convert hard-coded values into Helm variables (see step 4).

Edit helm/charts/auth-api/Chart.yaml to set name/version:

apiVersion: v2
name: auth-api
description: Auth API service
version: 0.1.0
appVersion: "0.1.0"


Keep each subchart small — only templates for that microservice.

3) Create the umbrella chart

Create helm/umbrella/Chart.yaml:

apiVersion: v2
name: my-app
version: 0.1.0
dependencies:
- name: auth-api
  version: 0.1.0
  repository: "file://../charts/auth-api"
  condition: auth-api.enabled
- name: user-api
  version: 0.1.0
  repository: "file://../charts/user-api"
  condition: user-api.enabled


condition allows toggling each subchart on/off from umbrella/values.yaml.

Create helm/umbrella/values.yaml with global and per-subchart blocks:

global:
namespace: dev
ingress:
className: nginx
baseDomain: "dev.example.com"

auth-api:
enabled: true
replicaCount: 2
image:
repository: jkole822/devops-project-auth-api
tag: latest
secretName: auth-secret-dev

user-api:
enabled: true
replicaCount: 1
image:
repository: jkole822/devops-project-user-api
tag: latest


When you install the umbrella chart it will read these and pass per-subchart values.

4) Convert your YAMLs to templates and wire values

In helm/charts/auth-api/templates/deployment.yaml (example):

apiVersion: apps/v1
kind: Deployment
metadata:
name: {{ include "auth-api.fullname" . }}
labels:
app: {{ .Values.labels.app | default "devops-project" }}
spec:
replicas: {{ .Values.replicaCount | default 1 }}
selector:
matchLabels:
app: {{ .Values.labels.app | default "devops-project" }}
template:
metadata:
labels:
app: {{ .Values.labels.app | default "devops-project" }}
spec:
containers:
- name: {{ .Chart.Name }}
image: "{{ .Values.image.repository }}:{{ .Values.image.tag | default .Chart.AppVersion }}"
ports:
- containerPort: {{ .Values.containerPort | default 8080 }}
envFrom:
- secretRef:
name: {{ .Values.secretName | default "auth-secret" }}


And helm/charts/auth-api/values.yaml:

replicaCount: 1
image:
repository: jkole822/devops-project-auth-api
tag: latest
containerPort: 8080
secretName: auth-secret
labels:
app: devops-project


Repeat for service.yaml, ingress.yaml etc. Use .Values for anything you might want to change (image tag, replicas, ports, hostnames, secrets, resources).

Tip: Use {{ .Release.Name }} or the builtin helper include "auth-api.fullname" . (created by helm create) to keep names unique per release.

5) Make umbrella pass values into subcharts

The umbrella values.yaml keys must match the subchart top-level key names (like auth-api:). Helm will merge the umbrella values into the subchart’s .Values.

umbrella/values.yaml example already shows this. To override production values, add umbrella/values-prod.yaml and use -f at install time.

6) Install locally / test rendering

From umbrella dir:

cd helm/umbrella
helm dependency update    # copies the file:// charts into umbrella/charts/
helm lint ..              # lint umbrella and all subcharts (run from repo root)
helm template my-app . -f values.yaml    # render YAML locally (good for review)


To install into your cluster (dev):

helm install my-app . -f values.yaml --namespace dev --create-namespace


To upgrade:

helm upgrade my-app . -f values.yaml --namespace dev


To uninstall:

helm uninstall my-app --namespace dev

7) Upgrade just one subchart (when you need per-service deploys)

When umbrella is installed as release my-app, each subchart is installed as a child release named my-app-<subchart> (e.g., my-app-auth-api). You can upgrade only that child:

# upgrade only auth-api child release using the local chart
helm upgrade my-app-auth-api ../charts/auth-api --reuse-values --set image.tag=1.2.3


Or package the subchart and upgrade:

cd helm/charts/auth-api
helm package .
# move the tgz into umbrella/charts/ or use the folder directly
helm upgrade my-app-auth-api ./auth-api-0.1.0.tgz --install --reuse-values


(Using child release upgrades is handy, but keep umbrella values in sync if umbrella controls that subchart too.)

8) CI/CD workflow (simple example)

Build Docker image, tag with commit SHA, push to registry (ECR/GCR/DockerHub).

Run tests.

Update helm values or use --set <chart>.image.tag=$SHA.

Run helm dependency update helm/umbrella

helm upgrade --install my-app helm/umbrella -f helm/umbrella/values-prod.yaml --namespace prod

You can automate the above in GitHub Actions, GitLab CI, or any pipeline.

9) EKS-specific notes (short)

Use global.ingress.className in umbrella values to switch between nginx and alb (AWS Load Balancer Controller).

If you use ALB & cloud resources, create the necessary IAM roles / IRSA for controllers (aws-load-balancer-controller) — tools like eksctl make this easier.

For storage use storageClassName: gp2 or whichever class EKS uses; keep storage settings in values so they are environment specific.

Use Namespace-level ResourceQuotas / LimitRanges in prod and supply resources.requests/limits in subcharts.

10) Extras & best practices

Keep secrets out of values.yaml. Use Kubernetes Secrets created independently, or use helm-secrets, SOPS, or external secret managers (ExternalSecrets).

Use helm lint and helm template often while converting manifests.

Use condition or tags on dependencies if you want to enable/disable subcharts from umbrella values.

Version your charts (bump version in Chart.yaml`) and consider a chart repo (ChartMuseum or GitHub Pages) if you publish charts.

Add NOTES.txt in each chart (templates/NOTES.txt) to display helpful info after install.

Example quick checklist (do this for each service)

helm create <service> → remove defaults.

Move k8s/<service>/*.yaml → helm/charts/<service>/templates/.

Replace hard-coded values with {{ .Values.* }} and update values.yaml.

Add dependency entry in helm/umbrella/Chart.yaml with condition: <service>.enabled.

Add per-service block in helm/umbrella/values.yaml.

helm dependency update helm/umbrella → helm lint → helm template → helm install/upgrade.