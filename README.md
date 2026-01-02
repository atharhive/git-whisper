# Git Whisperer
Turn commit history into a human story

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

### First Time Setup
Git Whisperer includes an **interactive setup wizard** that guides you through configuration:

```bash
# Clone and enter the repository
git clone <repository-url>
cd git-whisperer

# Run Git Whisperer (first run will guide you through setup)
python main.py /path/to/your/git/repo
```

The setup wizard will:
- ✅ Prompt for your Gemini API key
- ✅ Test MongoDB connection
- ✅ Offer to start local MongoDB with Docker if needed
- ✅ Save configuration to `.env` for future runs

### Manual Setup (Optional)
If you prefer manual configuration:

1. Copy the environment template:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env` with your configuration:
   ```bash
   GEMINI_API_KEY=your_actual_api_key_here
   MONGODB_URL=mongodb://localhost:27017/
   ```

### Running Analysis
```bash
# Analyze any git repository
python main.py /path/to/git/repo

# Analyze current directory
python main.py .
```

## 🔧 Configuration

Git Whisperer uses environment variables for configuration. The interactive setup creates a `.env` file with:

- `GEMINI_API_KEY`: Your Google Gemini API key
- `MONGODB_URL`: MongoDB connection string (default: `mongodb://localhost:27017/`)
- `MONGODB_DB`: Database name (default: `git_whisperer_db`)
- `LOG_LEVEL`: Logging verbosity (default: `INFO`)

See `.env.example` for all available options.

## 🎨 CLI Interface

Git Whisperer features a beautiful, colored CLI interface powered by [Rich](https://github.com/Textualize/rich):

- **Colored Logging**: Informative messages with emojis and colors
- **Progress Indicators**: Real-time status updates during analysis
- **Rich Panels**: Formatted output with borders and styling
- **Error Handling**: Clear error messages with troubleshooting tips

Example output includes:
- Repository analysis summary with commit counts
- AI-generated project stories in styled panels
- Progress bars for long-running operations
- Color-coded status messages (✅ success, ❌ errors, ⚠️ warnings)

### Development
For development with live code reloading:
```bash
docker-compose -f docker/compose.yaml -f docker-compose.override.yaml up --build
```

### Local Development (without Docker)
If you prefer to run locally:

1. **Install uv** (fast Python package manager):
   ```bash
   # On macOS
   brew install uv
   
   # Or install from https://github.com/astral-sh/uv
   ```

2. **Install dependencies**:
   ```bash
   uv pip install -r pyproject.toml
   ```

3. **Set up environment**:
   ```bash
   cp .env.example .env
   # Edit .env with your GEMINI_API_KEY
   ```

4. **Start MongoDB** (if running locally):
   ```bash
   brew install mongodb-community
   brew services start mongodb-community
   ```

5. **Run the application**:
   ```bash
   uv run python main.py /path/to/your/git/repo
   ```

## ✨ Features

- **🎯 Interactive Setup Wizard**: First-run configuration with API key and database setup
- **🤖 AI-Powered Storytelling**: Uses Google's Gemini AI to generate human-readable project narratives
- **💾 Smart Database Management**: Auto-detects and configures MongoDB (local Docker or remote)
- **🎨 Beautiful CLI Interface**: Rich, colored output with progress indicators and panels
- **📊 Git History Analysis**: Deep analysis of commit patterns and evolution
- **🐳 Docker Integration**: Automatic MongoDB container management
- **🔧 Flexible Configuration**: Environment-based configuration with automatic .env generation
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
