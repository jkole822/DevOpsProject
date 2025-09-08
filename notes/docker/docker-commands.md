# 🐳 Docker Commands Notes
1. `docker run`
   - Runs a container from an image.
   - If the image is not present locally, Docker pulls it from Docker Hub the first time.
   - Subsequent runs reuse the local image.
   - Example: `docker run nginx`
   - The `-it` flag enables interactive terminal access within a Docker container.
     - `-i` (or --interactive): This flag keeps the standard input (stdin) open, allowing you to send commands to the container. Without this, the container might exit immediately if it doesn't have a foreground process to attach to.
     - `-t` (or --tty): This flag allocates a pseudo-terminal (tty), providing a terminal interface within the container. This is crucial for interactive shells like bash or sh, as it allows for proper command-line editing, arrow key navigation, and colored output.
   - Name a container with `--name` flag followed by the desired name.
   - The `-p` flag can be used to map a host port to the port used within the docker container.
     - Example: `docker run -p 80:5000 webapp`
   - The `-v` flag allows you to map a local directory to a directory within a container. These are known as volumes and are useful to keep data separated from docker containers. This is useful since all data within a container is deleted when that container is deleted.
     - Example: `docker run -v /opt/datadir:/var/lib/mysql mysql`
2. `docker ps`
   - Lists running containers.
   - Shows: container ID, image name, status, container name, etc.
   - Containers get a random ID and name by default.
   - Example: docker ps
   - To list all containers (running + stopped): `docker ps -a`
3. `docker stop`
   - Stops a running container.
   - Must provide container ID or name.
   - Example: `docker stop silly_summit`
4. `docker rm`
   - Removes a stopped or exited container permanently.
   - Example: `docker rm silly_summit`
5. `docker images`
   - Lists all images available locally.
   - Shows: repository, tag, image ID, size.
   - Example: `docker images`
6. `docker rmi`
   - Removes an image.
   - Must ensure no containers are running from that image.
   - Example: `docker rmi nginx`
7. `docker pull`
   - Downloads an image without running a container.
   - Useful to prepare images ahead of time.
   - Example: `docker pull ubuntu`
8. **Containers and Processes**
   - Containers run a specific process (unlike VMs, which run entire OSes).
   - A container stops when its process stops.
   - Example: `docker run ubuntu`
   - Run with a specific command: `docker run ubuntu sleep 5`
9. `docker exec`
   - Run a command inside a running container.
   - Example: `docker exec container_id cat /etc/hosts`
10. **Running in Foreground vs. Detached**
    - Foreground (attached mode): `docker run cloud/simple-web-app`
      - Attached to console.
      - Logs stream directly to your terminal.
      - Use Ctrl+C to stop.
    - Detached mode (-d): `docker run -d cloud/simple-web-app`
      - Runs in the background.
      - You get your terminal back immediately.
    - Attach back to a running container: `docker attach container_id`
    - Container IDs can be shortened (first few characters, if unique).
11. `docker inspect`
    - Returns all details of a container in a JSON format, such as the state mounts, configuration data, and networking settings.
    - Example: `docker inspect container_name`
12. `docker logs`
    - Return content written to the standard out of that container.
    - Example: `docker logs container_name`

✅ These are the essential Docker CLI basics: running containers, listing, stopping, removing, pulling images, and running processes inside containers.
