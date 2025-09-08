# Advanced Docker Concepts: Storage Drivers and File Systems

## Where Docker Stores Data

- Default directory: `/var/lib/docker`
- Subfolders:
  - **containers/** → files related to containers
  - **image/** → files related to images
  - **volumes/** → volumes created by containers
  - **etc/** → other Docker metadata

---

## Layered Architecture of Images

- Docker builds images in **layers**.
- Each `Dockerfile` instruction = new layer with only the changes from the previous one.
- Example image layers:
  1. Base Ubuntu OS (e.g., 120 MB)
  2. Installed apt packages (~300 MB)
  3. Python/Flask dependencies (6MB)
  4. Application source code (230 B)
  5. Entry point configuration (0 B)

### Benefits

- **Layer reuse** → Faster builds & saves disk space.
- Updates only rebuild modified layers.
- Shared layers between multiple images/containers.

---

## Image Layers vs Container Layers

- **Image layers** → Read-only. Immutable after build.
- **Container layer (writable)** → Added on top when a container is created. Stores:
  - Logs
  - Temp files
  - User modifications

### Copy-on-Write Mechanism

- If a file in an image layer is modified:
  - Docker copies it into the writable container layer.
  - Modifications are applied to the copy, leaving the original unchanged.

### Lifecycle

- Writable layer exists only while the container runs.
- Destroying the container removes this layer and its data.

---

## Persisting Data with Volumes

### Volume Mounts

- Created under `/var/lib/docker/volumes/`.
- Example:
  ```bash
  docker volume create data_volume
  docker run -v data_volume:/var/lib/mysql mysql
  ```
- Data persists even after container deletion.
- If volume doesn’t exist, Docker auto-creates it.

### Bind Mounts

- Use existing host directory instead of Docker-managed volume.
- Example:

```bash
docker run -v /data/mysql:/var/lib/mysql mysql
```

- Useful for existing external storage.

**-v vs --mount**

- `-v` (short form) → legacy option.
- `--mount` (long form) → preferred, more explicit.
- Example:

```bash
docker run --mount type=bind,source=/data/mysql,target=/var/lib/mysql mysq
```

## Storage Drivers

- Responsible for:
  - Managing layered architecture
  - Writable layers
  - Copy-on-write behavior
- Common drivers:
  - AUFS (default on Ubuntu)
  - Overlay / Overlay2 (modern, widely used)
  - Btrfs
  - ZFS
  - Device Mapper
- Selection depends on OS and use case.
- Docker automatically picks the best available driver.

## Key Takeaways

- Docker stores its data in /var/lib/docker with separate folders for containers, images, and volumes.
- Images are built in immutable layers; containers add a writable layer.
- Copy-on-write allows modifying files without changing the base image.
- Data persistence requires volumes (Docker-managed) or bind mounts (host directories).
- Storage drivers implement layered file systems and resource management.
