# Git Whisperer - Complete Workflow Guide

A step-by-step guide to get Git Whisperer up and running on your machine.

---

## 📋 Prerequisites

Before you begin, ensure you have the following installed on your machine:

### 1. IDE / Code Editor
- **Visual Studio Code** (Recommended)
  - Download: [https://code.visualstudio.com/download](https://code.visualstudio.com/download)
  - Choose the version for your operating system (Windows, macOS, or Linux)

### 2. Docker Desktop
- **Docker Desktop** (Required for MongoDB)
  - Download: [https://www.docker.com/products/docker-desktop](https://www.docker.com/products/docker-desktop)
  - Install and ensure Docker is running before proceeding

### 3. Rust Toolchain
You need Rust, Cargo, and the nightly toolchain installed:

**Install Rust and Cargo:**
```bash
# On Windows, macOS, or Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Or visit: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

**Install Rust Nightly:**
```bash
rustup install nightly
rustup default nightly
```

**Verify Installation:**
```bash
rustc --version
cargo --version
rustup show
```

### 4. Google Gemini API Key
- Get your **free** API key from: [https://makersuite.google.com/app/apikey](https://makersuite.google.com/app/apikey)
- Sign in with your Google account
- Click "Create API Key" and copy it (starts with `AIza...`)
- **Note:** Currently optimized for **Gemini 2.5 Flash** model (free tier)

---

## 🚀 Installation Steps

### Step 1: Clone the Repository
```bash
git clone <repository-url>
cd git-whisperer
```

### Step 2: Build the Project
```bash
cargo build --release
```

This will compile the Rust code and create an optimized binary in `./target/release/whisper`

**Build time:** First build may take 5-10 minutes as it downloads and compiles dependencies.

### Step 3: Start Docker Desktop
- Open Docker Desktop application
- Ensure it's running (you should see the Docker icon in your system tray)

### Step 4: Start MongoDB with Docker
```bash
docker-compose -f docker/compose.yaml up -d
```

This command:
- Starts MongoDB in a Docker container
- Runs in detached mode (background)
- Sets up the database automatically

**Verify Docker is running:**
```bash
docker ps
```

You should see containers for MongoDB and the Git Whisperer app.

### Step 5: Configure Environment Variables
Create a `.env` file in the project root:

```bash
cp .env.example .env
```

Edit the `.env` file and add your Gemini API key:

```env
# Your actual Gemini API key
GEMINI_API_KEY=AIzaSyC_your_actual_key_here

# MongoDB URL (use this for Docker setup)
MONGODB_URL=mongodb://admin:password@localhost:27017/

# Database configuration
MONGODB_DB=git_whisperer_db
MONGODB_COLLECTION=project_history

# Logging level
LOG_LEVEL=INFO
```

**Important:** 
- Replace `your_gemini_api_key_here` with your actual API key
- Keep the `.env` file private (never commit it to git)
- The `.env` file must be exactly as shown, only change the API key

---

## 🎯 Usage Workflow

### Basic Commands

Once setup is complete, you can use Git Whisperer from the `./target/release/` directory:

#### 1. Run Setup Wizard (Optional)
```bash
./target/release/whisper setup
```

This interactive wizard helps you configure:
- Gemini API key
- MongoDB connection
- Database settings

#### 2. Analyze a Repository
```bash
# Add and analyze a repository from URL
./target/release/whisper add https://github.com/username/repo

# Or analyze a local repository
./target/release/whisper add /path/to/local/repo
```

#### 3. Generate Project Summary
```bash
./target/release/whisper summary
```

Generates a human-readable story of the project's evolution.

#### 4. Create Demo Script
```bash
./target/release/whisper demo
```

Creates a 60-90 second demo narration script.

#### 5. Analyze Recent Commits
```bash
# Last 5 commits (default)
./target/release/whisper last

# Last 10 commits
./target/release/whisper last 10
```

#### 6. Changes Since a Reference
```bash
# Since a specific commit
./target/release/whisper since abc1234

# Since a tag
./target/release/whisper since v1.0.0

# Since a date
./target/release/whisper since "2024-01-01"
```

#### 7. Generate Changelog
```bash
./target/release/whisper changelog
```

Creates a clean, categorized changelog.

#### 8. Quick Analysis (One Command)
```bash
./target/release/whisper https://github.com/username/repo
```

Adds the repository and immediately shows the summary.

---

## 📝 Complete Workflow Example

Here's a typical workflow from start to finish:

```bash
# 1. Clone Git Whisperer
git clone <repository-url>
cd git-whisperer

# 2. Build the project
cargo build --release

# 3. Start Docker Desktop (GUI)

# 4. Start MongoDB
docker-compose -f docker/compose.yaml up -d

# 5. Configure .env file with your API key
cp .env.example .env
# Edit .env and add your GEMINI_API_KEY

# 6. Analyze a repository
./target/release/whisper add https://github.com/microsoft/vscode

# 7. Generate outputs
./target/release/whisper summary
./target/release/whisper demo
./target/release/whisper changelog

# 8. Analyze recent work
./target/release/whisper last 10
```

---

## ⚠️ Important Notes

### Repository Size Limitations
- **Do not analyze repositories with more than 500 commits**
- Large repositories may take a long time to process
- The Gemini API has rate limits (15 requests/minute on free tier)
- For best results, use repositories with 50-300 commits

### API Key Requirements
- Currently optimized for **Gemini 2.5 Flash** model
- Free tier is sufficient for most use cases
- Rate limits: 15 requests/minute, 1,500 requests/day

### Docker Requirements
- Docker Desktop must be running before starting MongoDB
- MongoDB container must be running for the app to work
- Check container status with: `docker ps`

---

## 🛠️ Troubleshooting

### Build Errors
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Docker Issues
```bash
# Check if Docker is running
docker ps

# Restart containers
docker-compose -f docker/compose.yaml down
docker-compose -f docker/compose.yaml up -d

# View logs
docker-compose -f docker/compose.yaml logs -f
```

### MongoDB Connection Issues
- Ensure Docker Desktop is running
- Check if MongoDB container is up: `docker ps`
- Verify `.env` file has correct `MONGODB_URL`
- For Docker setup, use: `MONGODB_URL=mongodb://admin:password@localhost:27017/`

### API Key Issues
- Verify your API key is correct in `.env`
- Check you haven't exceeded rate limits
- Ensure you have internet connection
- Test at: [https://makersuite.google.com/app/apikey](https://makersuite.google.com/app/apikey)

---

## 🎉 You're Ready!

Once you've completed all the steps above, you can start analyzing repositories and generating beautiful project narratives with Git Whisperer.

For more detailed information, see:
- [README.md](README.md) - Full project documentation
- [SETUP_GUIDE.md](SETUP_GUIDE.md) - Detailed setup instructions
- [docker/README.md](docker/README.md) - Docker-specific guide

Happy analyzing! 🚀
