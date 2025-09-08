# Jenkins

## Installation
```
wget -q -O - https://pkg.jenkins.io/debian-stable/jenkins.io.key | sudo apt-key add -
sudo sh -c 'echo deb https://pkg.jenkins.io/debian-stable binary/ > /etc/apt/sources.list.d/jenkins.list'
sudo apt update
sudo apt install openjdk-17-jdk -y
sudo apt install jenkins -y
sudo systemctl start jenkins
sudo systemctl enable jenkins
```

- `wget -q -O - ...` → downloads the Jenkins signing key and sends it to stdout (the -O - part).
- `| sudo apt-key add -` → pipes that key into apt-key, so your system trusts packages signed by Jenkins.
  - 👉 Without this, apt would refuse to install Jenkins, since it wouldn’t trust the source.
- `echo deb ...` → creates a new repository entry pointing to Jenkins’ Debian/Ubuntu package repo.
- `/etc/apt/sources.list.d/jenkins.list` → saves that entry in a new file dedicated to Jenkins.
  - 👉 Now apt knows where to look for Jenkins.
- `sudo apt update` → Refreshes your package lists so that apt is aware of Jenkins packages from the new repo.
- `sudo apt install openjdk-17-jdk -y` → Installs Java SDK which is a dependency of Jenkins 
  - Ensure `/etc/default/jenkins` has JAVA_HOME set to output from `readlink -f /usr/bin/java`
- `sudo apt install jenkins -y` → Installs Jenkins.
  - The `-y` flag auto-confirms prompts (so you don’t have to type y).
- `sudo systemctl start jenkins` → Starts the Jenkins service immediately.
- `sudo systemctl enable jenkins` → Ensures Jenkins starts automatically on boot.
- At this point, Jenkins is installed and running as a service.
- The last step usually is to access Jenkins via your browser:
  - 👉 http://<your-ec2-public-ip>:8080

## Setup
1️⃣ Security Group (AWS Firewall)

1. Go to the EC2 Console → Instances → Select your instance.
2. Scroll to Security → Security Groups → Click the group.
3. Under Inbound rules, click Edit inbound rules.
4. Add a rule:
   - Type: Custom TCP
   - Port range: 8080
   - Source: your IP (or 0.0.0.0/0 if you want open access — less secure)
5. Save changes.

✅ This tells AWS to allow traffic to your instance on port 8080.

2️⃣ OS Firewall (UFW on Ubuntu, optional)

Check if UFW is active:
`sudo ufw status`

If inactive, you’re fine.

If active, allow 8080:
```commandline
sudo ufw allow 8080/tcp
sudo ufw reload
```

## Implementation
```
pipeline {
    agent {
        any {
            retries 2
        }
    }

    stages {
        stage('Checkout') {
            steps {
                git branch: 'main', url: 'https://github.com/jkole822/DevOpsProject.git'
            }
        }

        stage('Install Dependencies') {
            steps {
                sh 'python3 -m venv venv'
                sh './venv/bin/pip install --upgrade pip'
                sh './venv/bin/pip install -r requirements.txt'
                sh './venv/bin/pip install pytest'
            }
        }

        stage('Run Tests') {
            steps {
                sh './venv/bin/pytest'
            }
        }
    }
}
```
- `python3 -m venv venv` → calls Python’s built-in module to create a virtual environment.
  - `venv` → the folder name where the virtual environment will be stored (you can name it anything).
  - Purpose: isolates your project’s Python packages from system-wide Python, so dependencies don’t conflict with other projects.
  - Outside of Jenkins, you can use `source venv/bin/activate` to activate the virtual environment.
    - Activating the environment changes your PATH so that python and pip now point to the virtual environment instead of the system Python. 
    - You’ll usually see (venv) prepended to your terminal prompt when activated.
    - Jenkins starts a new shell for each sh, so the first `source` doesn’t persist to the next sh.
      - `source` (built-in shell [Bash/Zsh] command) → It runs a script in the current shell context instead of spawning a new shell.
- In Declarative syntax, you can use the retries option on the agent to automatically retry failed stages
  - If the git step fails due to temporary issues or controller restarts, Jenkins will retry automatically.

## Jenkins Pipeline / Job Settings Explained

### 1. Do not allow concurrent builds
- **What it does:** Prevents a new build from starting if a previous build is still running.  
- **Use case:** Avoid race conditions or multiple builds overwriting each other.  
- **Example:** Deployments to a staging or production server.

---

### 2. Do not allow the pipeline to resume if the controller restarts
- **What it does:** Prevents Jenkins from resuming the pipeline automatically after a controller restart.  
- **Use case:** Some steps (e.g., `git`) cannot resume after restart. Avoid skipped stages or silent failures.  

---

### 3. GitHub project
- **What it does:** Sets the URL of the associated GitHub repository.  
- **Use case:** Adds a convenient link on the Jenkins job page.  
- **Note:** Informational only; doesn’t affect builds unless GitHub hooks are used.

---

### 4. Pipeline speed/durability override
- **What it does:** Controls how pipeline state is persisted to disk.  
- **Options:**  
  - Maximum durability: safest, survives crashes or restarts.  
  - Fast: faster execution, but pipeline state may be lost if Jenkins restarts mid-build.  
- **Use case:**  
  - Fast pipelines for short-lived tasks.  
  - Maximum durability for critical or long-running builds.

---

### 5. Preserve stashes from completed builds
- **What it does:** Keeps stashed files after build completion.  
- **Use case:** Useful for debugging or analyzing files passed between stages.  
- **Note:** Normally, stashes are temporary.

---

### 6. This project is parameterized
- **What it does:** Allows defining input parameters for builds.  
- **Use case:** Pass variables like branch name, environment, or feature flags into the pipeline.  
- **Example:**
```groovy
parameters {
    string(name: 'BRANCH', defaultValue: 'main', description: 'Branch to build')
}

stages {
    stage('Checkout') {
        steps {
            git branch: "${params.BRANCH}", url: 'https://github.com/jkole822/DevOpsProject.git'
        }
    }
}
```

### 7. Throttle builds
- What it does: Limits the number of builds that can run concurrently, either globally or per node.
- Use case:
  - You have limited servers/resources and don’t want too many builds running at once. 
  - Avoids overloading agents or deployment targets.

## Jenkins Job Trigger Options

### 1. Build after other projects are built
- **What it does:** Triggers this job **after one or more other Jenkins jobs finish successfully**.  
- **Use case:**  
  - You have dependent jobs, e.g., a deployment job that should run after a successful build job.  
- **Example:**  
  - `Build Project B` automatically starts when `Build Project A` finishes.

---

### 2. Build periodically
- **What it does:** Schedules the job to run at regular intervals using a cron-like syntax.  
- **Use case:**  
  - Run nightly builds, automated tests, or regular maintenance tasks.  
- **Example:**  
  - `H 2 * * *` → Run every day at approximately 2 AM.

---

### 3. GitHub hook trigger for GITScm polling
- **What it does:** Triggers the job **automatically when changes are pushed to GitHub**, using webhooks.  
- **Use case:**  
  - Continuous Integration: automatically build when someone pushes code.  
- **Notes:**  
  - Requires your GitHub repository to be configured with a webhook pointing to your Jenkins server.

---

### 4. Poll SCM
- **What it does:** Jenkins **periodically checks the repository** for changes.  
- **Use case:**  
  - Alternative to webhooks if your Git host doesn’t support them.  
- **Example:**  
  - `H/15 * * * *` → Check for changes every 15 minutes, and build if there are updates.  

---

### 5. Trigger builds remotely (e.g., from scripts)
- **What it does:** Allows starting a Jenkins build **via an HTTP request**.  
- **Use case:**  
  - Trigger builds from external scripts, CI/CD pipelines, or other systems.  
- **Notes:**  
  - Requires a **token** to authenticate the request.  
  - Example URL:  
    ```
    http://<JENKINS_URL>/job/<JOB_NAME>/build?token=MY_TOKEN
    ```
    
---
