<p align="center">
  <div>
    <img src="https://upload.wikimedia.org/wikipedia/commons/c/c3/Python-logo-notext.svg" alt="Python" width="50"/>
    <br>Python
  </div>
  <div>
    <img src="https://cdn.worldvectorlogo.com/logos/mongodb-icon-1.svg" alt="MongoDB" width="50"/>
    <br>MongoDB
  </div>
  <div>
    <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d7/Google_Gemini_logo_2023.svg/200px-Google_Gemini_logo_2023.svg.png" alt="Google Gemini" width="50"/>
    <br>Gemini AI
  </div>
  <div>
    <img src="https://git-scm.com/images/logos/downloads/Git-Icon-1788C.png" alt="Git" width="50"/>
    <br>Git
  </div>
  <div>
    <img src="https://www.svgrepo.com/show/513512/command-line.svg" alt="CLI" width="50"/>
    <br>CLI
  </div>
</p>

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

## Problem
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
