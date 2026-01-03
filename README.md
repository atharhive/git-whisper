# Git Whisper
Turn commit history into a human story

> **Now written in Rust** for blazing-fast performance and single-binary distribution!

**Binary Name**: `whisper` (not `git-whisperer`)

## Vision
Git Whisperer exists to solve a quiet but universal hacker problem:
we build things faster than we can explain them.
Commits are written for machines and teammates-in-the-moment.
Demos, READMEs, and changelogs are written under pressure, often at the last minute.
Git Whisperer reads the development history of a project and translates it into plain English:
what problem the project tried to solve, how it evolved, and what actually matters.
This is not code analysis.
This is engineering storytelling.

## 🚀 Quick Start

### Option 1: Docker (Recommended)

```bash
# Clone the repository
git clone <repository-url>
cd git-whisperer

# Start with Docker (includes MongoDB)
docker-compose -f docker/compose.yaml up -d

# Run setup wizard
docker-compose -f docker/compose.yaml exec app whisper setup

# Analyze a repository
docker-compose -f docker/compose.yaml exec app whisper add <repo-url>
docker-compose -f docker/compose.yaml exec app whisper summary
```

### Option 2: Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd git-whisperer

# Build release binary
cargo build --release

# Run setup wizard
./target/release/whisper setup

# Analyze a repository
./target/release/whisper add <repo-url>
./target/release/whisper summary
./target/release/whisper summary
```

### Option 3: Install via Cargo

```bash
# Install globally
cargo install --git https://github.com/atharhive/git-whisperer

# Run setup
whisper setup

# Use
whisper add <repo-url>
whisper summary
```

## 🔧 Configuration

Git Whisperer uses environment variables for configuration. The interactive setup creates a `.env` file with:

- `GEMINI_API_KEY`: Your Google Gemini API key ([Get one here](https://makersuite.google.com/app/apikey))
- `MONGODB_URL`: MongoDB connection string
  - Docker: `mongodb://mongodb:27017/` (internal container networking)
  - Local: `mongodb://localhost:27017/`
  - Atlas: `mongodb+srv://user:pass@cluster.mongodb.net/...`
- `MONGODB_DB`: Database name (default: `git_whisperer_db`)
- `MONGODB_COLLECTION`: Collection name (default: `project_history`)
- `LOG_LEVEL`: Logging verbosity (default: `INFO`)

### Docker Configuration

When using Docker, the setup automatically configures the correct MongoDB URL for container networking. The `.env` file is mounted into the container.

For development with live code reloading:
```bash
docker-compose -f docker/compose.yaml -f docker-compose.override.yaml up -d
```

See [docker/README.md](docker/README.md) for detailed Docker setup instructions and [SETUP_GUIDE.md](SETUP_GUIDE.md) for manual configuration.

## 🎨 CLI Interface

Git Whisperer features a beautiful, colored CLI interface with automatic setup prompting:

```bash
whisper --help
```

**Available Commands:**
- `whisper setup` - Interactive setup wizard (runs automatically if needed)
- `whisper add <repo>` - Add/clone and analyze a repository
- `whisper summary` - Generate full project story from git history
- `whisper demo` - Create a 60-90 second demo script
- `whisper last [count]` - Explain recent commits (default: 5)
- `whisper since <ref>` - Changes since commit/tag/date
- `whisper changelog` - Generate clean changelog by type
- `whisper <repo-url>` - Quick mode: add repo and show summary

**Example Usage:**
```bash
# Quick analysis
whisper https://github.com/microsoft/vscode

# Step-by-step workflow
whisper add https://github.com/microsoft/vscode
whisper summary
whisper demo
whisper changelog

# Analyze recent work
whisper last 10
whisper since v1.0.0
whisper since "2024-01-01"
```

**Features:**
- **Colored Output**: Informative messages with emojis and colors
- **Progress Indicators**: Real-time spinners during analysis
- **Formatted Panels**: Clean, bordered output for results
- **Error Handling**: Clear error messages with troubleshooting tips
- **Auto-Setup**: Automatically prompts for configuration if not set up

## 🦀 Why Rust?

This is a complete rewrite from Python with significant improvements:

| Feature | Python | Rust |
|---------|--------|------|
| Startup time | ~500ms | ~10ms |
| Distribution | Requires Python + deps | Single binary |
| Performance | Good | Excellent |
| Memory usage | Higher | Lower |
| Cross-platform | pip install | Compile once, run anywhere |

The Rust version is production-ready and can be distributed via `cargo install` or as a standalone binary.

## 🐳 Docker Deployment

Git Whisperer includes complete Docker support for easy deployment:

### Quick Docker Setup
```bash
# Full stack with MongoDB
docker-compose -f docker/compose.yaml up -d

# Development with live reloading
docker-compose -f docker/compose.yaml -f docker-compose.override.yaml up -d

# Run commands
docker-compose -f docker/compose.yaml exec app whisper add <repo-url>
```

### Docker Benefits
- **Zero Configuration**: Everything included and configured
- **Database Included**: MongoDB runs automatically in container
- **Development Ready**: Volume mounts for live code reloading
- **Production Ready**: Multi-stage builds for minimal images
- **Cross-Platform**: Same setup on Linux, Mac, Windows

See [docker/README.md](docker/README.md) for detailed Docker instructions.

## ✨ Features

- **🚀 Blazing Fast**: Written in Rust for maximum performance
- **📦 Single Binary**: No dependencies, just download and run
- **🐳 Docker Ready**: Complete containerized setup with MongoDB
- **🎯 Auto-Setup Wizard**: First-run configuration with API key and database setup
- **🌐 URL Support**: Analyze repos directly from GitHub/GitLab URLs or local paths
- **🤖 AI-Powered Storytelling**: Uses Google's Gemini AI to generate human-readable project narratives
- **💾 Flexible Database**: Supports local MongoDB (Docker), MongoDB Atlas (cloud), or custom instances
- **🎨 Beautiful CLI Interface**: Colored output with progress indicators and formatted panels
- **📊 Git History Analysis**: Deep analysis of commit patterns and evolution using libgit2
- **🔧 Smart Configuration**: Environment-based configuration with automatic .env generation
- **🔒 Secure**: Password masking in output, no credentials exposed
- **⚡ Quick Mode**: Single command to analyze and summarize any repository
## 🛠️ Development

### Local Development
```bash
# Run in dev mode
cargo run -- /path/to/repo

# Run setup
cargo run -- setup

# Build release
cargo build --release

# Run tests
cargo test

# Check code
cargo check
```

### Docker Development
```bash
# Start development environment
docker-compose -f docker/compose.yaml -f docker-compose.override.yaml up -d

# Run commands in container
docker-compose -f docker/compose.yaml exec app whisper setup
docker-compose -f docker/compose.yaml exec app whisper add <repo-url>

# View logs
docker-compose -f docker/compose.yaml logs -f app

# Stop services
docker-compose -f docker/compose.yaml down
```

### Building Docker Image
```bash
# Build for local testing
docker build -f docker/Dockerfile -t whisper .

# Run locally
docker run -it --rm \
  -e GEMINI_API_KEY=your_key \
  -e MONGODB_URL=mongodb://host:port \
  whisper --help
```

## 📁 Project Structure

```
git-whisperer/
├── src/
│   ├── main.rs           # CLI entry point with clap
│   ├── cli/              # Command handlers
│   │   ├── setup.rs      # Interactive setup wizard
│   │   ├── add.rs        # Repository addition
│   │   ├── analyze.rs    # Repository analysis
│   │   ├── summary.rs    # Project summary generation
│   │   ├── demo.rs       # Demo script creation
│   │   ├── last.rs       # Recent commits analysis
│   │   ├── since.rs      # Changes since reference
│   │   ├── changelog.rs  # Changelog generation
│   │   └── mod.rs        # Module declarations
│   ├── repository.rs     # Git parsing with libgit2
│   ├── gemini.rs         # Gemini API client
│   ├── storage.rs        # MongoDB operations
│   ├── config.rs         # Configuration management
│   └── workspace.rs      # Workspace management
├── docker/
│   ├── compose.yaml      # Full stack (app + mongodb)
│   ├── core/compose.yaml # App only (requires external mongodb)
│   ├── mongo/compose.yaml# MongoDB only
│   ├── Dockerfile        # Rust application build
│   └── README.md         # Docker setup guide
├── Cargo.toml            # Rust dependencies
├── Cargo.lock            # Dependency lock file
├── .env.example          # Environment template
├── .dockerignore         # Docker build exclusions
├── docker-compose.override.yaml # Development overrides
├── README.md             # This file
└── SETUP_GUIDE.md        # Detailed setup instructions
```

## 🐍 Python Version

The original Python implementation is preserved in `python-legacy/` for reference. The Rust version is feature-complete and recommended for all use cases.

- Repos grow faster than documentation
- Hackers forget why decisions were made
- Demos become fragile explanations instead of clear narratives
- README files are often written after the fact and feel disconnected from reality
- Git already contains the truth — it’s just fragmented.

## Solution
Git Whisperer is a local-first AI tool that:
- Reads commit history and PR-like context
- Builds a structured memory of the project’s evolution
- Uses AI to extract intent, milestones, and impact
- Outputs human-readable artifacts instantly

No CI. No GitHub auth. No cloud dependency for ingestion.

## Core Features (MVP)
1.  **Commit History Analyzer**
    - Parses local git repositories using `git log`
    - Extracts:
        - Commit messages
        - File change summaries
        - Timestamps (chronology matters)
    - Ignores raw code to stay fast and reliable

2.  **Project Memory Store (MongoDB)**
    - Stores commits as structured “events”
    - Groups changes into phases:
        - Setup
        - Feature additions
        - Fixes
        - Refactors
    - This becomes long-term project memory, not just logs

3.  **AI Story Engine (Gemini)**
    - From commit history alone, Git Whisperer generates:
        - A plain-English project summary
        - A clean, readable CHANGELOG
        - A 60–90 second demo narration script
    - The AI is prompted to explain intent and evolution, not implementation details.

4.  **Instant Outputs**
    - README summary section
    - `CHANGELOG.md` draft
    - Demo script you can literally read during submission

## How It Works (Step-by-Step)
1.  User runs Git Whisperer locally inside a git repo
    - Example: `whisper add <repo-url>` or `whisper summary`
2.  Git Whisperer executes:
    - `git log --oneline --stat`
3.  Parses commit messages and change stats
4.  Parsed data is structured into events and stored in MongoDB:
    - Each commit becomes a timeline entry
    - Related commits are grouped by intent
5.  Structured history is sent to Gemini with a storytelling-focused prompt
6.  Gemini generates:
    - Project narrative
    - Changelog
    - Demo script
7.  Results are displayed in a beautiful CLI interface

## Why Local-First?
- Works offline
- No GitHub permissions required
- Faster iteration for hackathons
- More honest: focuses on what’s actually in the repo
This also makes Git Whisperer usable in private repos and early-stage projects.

## Example Output
**Project Summary**
“This project evolved from an initial scaffold into a functional application with authentication, performance optimizations, and a refined developer experience.”

**Changelog Entry**
“Added JWT-based authentication and refactored middleware to support scaling.”

**Demo Script Excerpt**
“I started with a basic scaffold, then focused on user authentication. After hitting performance issues, I refactored the core logic, which shaped the final architecture.”

## What Makes Git Whisperer Different
- Reads history, not static code
- Explains `why`, not just `what`
- Designed for demos, hackathons, and open source
- Turns engineering exhaust into clarity

This is a tool hackers immediately understand because it solves their own pain.

## Future Scope (Post-Hackathon)
- GitHub PR ingestion
- Visual timeline of project evolution
- Team-level narrative (who did what, when)
- Auto-generated release notes per tag
- Continuous “project memory” over time

## One-Line Summary (for judges)
Git Whisperer turns git history into human stories — READMEs, changelogs, and demo scripts generated straight from how a project actually evolved.
