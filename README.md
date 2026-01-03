# Git Whisperer
Turn commit history into a human story

> **Now written in Rust** for blazing-fast performance and single-binary distribution!

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

### Build from Source
```bash
# Clone the repository
git clone <repository-url>
cd git-whisperer

# Build release binary
cargo build --release

# Run setup wizard
./target/release/git-whisperer setup

# Analyze a repository
./target/release/git-whisperer /path/to/your/repo
```

### First Time Setup
Git Whisperer includes an **interactive setup wizard** that guides you through configuration:

```bash
git-whisperer setup
```

The setup wizard will:
- ✅ Prompt for your Gemini API key
- ✅ Test the API connection
- ✅ Let you choose MongoDB setup (local Docker, Atlas cloud, or custom)
- ✅ Test the database connection
- ✅ Save configuration to `.env` for future runs

See [SETUP_GUIDE.md](SETUP_GUIDE.md) for detailed instructions on getting API keys and MongoDB setup.

### Running Analysis
```bash
# Analyze any git repository (local path)
git-whisperer /path/to/git/repo

# Analyze current directory
git-whisperer .

# Clone and analyze from URL (GitHub, GitLab, etc.)
git-whisperer https://github.com/username/repo
git-whisperer git@github.com:username/repo.git

# Or use the explicit command
git-whisperer analyze /path/to/repo
```

## 🔧 Configuration

Git Whisperer uses environment variables for configuration. The interactive setup creates a `.env` file with:

- `GEMINI_API_KEY`: Your Google Gemini API key ([Get one here](https://makersuite.google.com/app/apikey))
- `MONGODB_URL`: MongoDB connection string
  - Local: `mongodb://localhost:27017/`
  - Atlas: `mongodb+srv://user:pass@cluster.mongodb.net/...`
- `MONGODB_DB`: Database name (default: `git_whisperer_db`)
- `MONGODB_COLLECTION`: Collection name (default: `project_history`)
- `LOG_LEVEL`: Logging verbosity (default: `INFO`)

See `.env.example` for all available options and [SETUP_GUIDE.md](SETUP_GUIDE.md) for detailed setup instructions.

## 🎨 CLI Interface

Git Whisperer features a beautiful, colored CLI interface:

- **Colored Output**: Informative messages with emojis and colors
- **Progress Indicators**: Real-time spinners during analysis
- **Formatted Panels**: Clean, bordered output for results
- **Error Handling**: Clear error messages with troubleshooting tips

Example output includes:
- Repository analysis summary with commit counts
- AI-generated project stories in styled panels
- Progress spinners for long-running operations
- Color-coded status messages (✅ success, ❌ errors, ⚠️ warnings)

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

## ✨ Features

- **🚀 Blazing Fast**: Written in Rust for maximum performance
- **📦 Single Binary**: No dependencies, just download and run
- **🌐 URL Support**: Analyze repos directly from GitHub/GitLab URLs or local paths
- **🎯 Interactive Setup Wizard**: First-run configuration with API key and database setup
- **🤖 AI-Powered Storytelling**: Uses Google's Gemini AI to generate human-readable project narratives
- **💾 Flexible Database**: Supports local MongoDB (Docker), MongoDB Atlas (cloud), or custom instances
- **🎨 Beautiful CLI Interface**: Colored output with progress indicators and formatted panels
- **📊 Git History Analysis**: Deep analysis of commit patterns and evolution using libgit2
- **🐳 Docker Integration**: Automatic MongoDB container management
- **🔧 Smart Configuration**: Environment-based configuration with automatic .env generation
- **🔒 Secure**: Password masking in output, no credentials exposed
## 🛠️ Development

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

## 📁 Project Structure

```
git-whisperer/
├── src/
│   ├── main.rs           # CLI entry point with clap
│   ├── cli/              # Command handlers
│   │   ├── setup.rs      # Interactive setup wizard
│   │   ├── analyze.rs    # Repository analysis
│   │   └── help.rs       # Help text
│   ├── repository.rs     # Git parsing with libgit2
│   ├── gemini.rs         # Gemini API client
│   ├── storage.rs        # MongoDB operations
│   └── config.rs         # Configuration management
├── python-legacy/        # Original Python implementation
├── SETUP_GUIDE.md        # Detailed setup instructions
└── Cargo.toml            # Rust dependencies
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
    - Example: `git-whisperer analyze`
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
7.  Results are displayed in a simple UI or CLI output

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
