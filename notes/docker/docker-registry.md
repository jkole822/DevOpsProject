# Docker Registry Notes

## What is a Registry?

- A **Docker registry** is where Docker images are stored.
- Analogy: if containers were rain, they would "rain" from registries (the cloud).
- Central repository of Docker images.

---

## Image Naming Convention

- Example: `docker run nginx`
  - `nginx` is the **repository name**.
  - Full form: `library/nginx`
    - `library/` prefix indicates **official images** when no user/organization is specified.
- If you create your own account or org on Docker Hub, images follow the pattern:

---

## Default Registry

- If no registry is specified, Docker assumes **Docker Hub** (`docker.io`).
- Actions:
- **Push** → upload images to the registry.
- **Pull** → download images from the registry.

---

## Other Popular Registries

- **Google Container Registry (GCR)** → `gcr.io`
- **Amazon ECR**, **Azure Container Registry**, **GitHub Container Registry**, etc.
- Public registries → contain open images accessible to everyone.
- Private registries → restrict access via credentials.

---

## Private Registries

- Cloud providers (AWS, GCP, Azure) provide private registries by default.
- You can set repositories as **private** → require credentials.
- Steps to use:

1. `docker login <registry>` → authenticate with username & password/token.
2. Run/pull images using the private registry prefix in the image name.
3. Without login → error: _image not found_.

---

## Running Your Own Private Registry

- Docker Registry itself is available as a containerized app.
- Image: `registry`
- Exposes API on port **5000**.

### Example: Run Local Registry

```bash
docker run -d -p 5000:5000 --name my-registry registry:2
```

---

## Pushing Images to Private Registry

1. Tag the image with registry info:

```bash
docker tag myapp:latest localhost:5000/myapp:latest
```

2. Push the image:

```bash
docker push localhost:5000/myapp:latest
```

---

## Pulling from Private Registry

- From the same host:

```bash
docker pull localhost:5000/myapp:latest
```

- From another host in the same network:

```bash
docker pull <docker-host-ip>:5000/myapp:latest
```
