#!/usr/bin/env python3
"""
Git Whisperer - Turn commit history into human stories
"""

import sys
import os
import logging
from pathlib import Path

# Add the app directory to Python path
sys.path.insert(0, str(Path(__file__).parent / 'app'))

from app.history_analyzer import get_git_log, parse_git_log
from app.memory_store import MemoryStore
from app.story_engine import StoryEngine
from app.config.envVars import EnvVars  # Import to trigger validation
from dotenv import load_dotenv

# Rich imports for beautiful CLI
from rich.console import Console
from rich.logging import RichHandler
from rich.panel import Panel
from rich.text import Text
from rich.progress import Progress, SpinnerColumn, TextColumn
from rich.prompt import Prompt, Confirm
from rich import print as rprint

# Load environment variables
load_dotenv()
EnvVars.reload()

# Create Rich console
console = Console()

# Configure rich logging (basic config, will be updated after setup)
logging.basicConfig(
    level=logging.INFO,
    format="%(message)s",
    datefmt="[%X]",
    handlers=[RichHandler(console=console, rich_tracebacks=True)]
)
logger = logging.getLogger(__name__)

def setup_configuration():
    """
    Interactive setup for required configuration.
    Prompts user for API keys and database configuration.
    """
    console.print("\n[bold blue]🔧 Git Whisperer Setup[/bold blue]\n")

    config = {}

    # Check for Gemini API Key
    gemini_key = os.getenv("GEMINI_API_KEY", "").strip()
    if not gemini_key:
        console.print("[yellow]🤖 Gemini API Key Required[/yellow]")
        console.print("Get your API key from: https://makersuite.google.com/app/apikey\n")

        gemini_key = Prompt.ask("Enter your Gemini API key", password=True)
        if not gemini_key:
            console.print("[red]❌ Gemini API key is required. Exiting...[/red]")
            sys.exit(1)
        config["GEMINI_API_KEY"] = gemini_key
    else:
        console.print("[green]✅ Gemini API key found[/green]")

    # Test Gemini API Key
    console.print("\n[yellow]🔍 Testing Gemini API connection...[/yellow]")
    try:
        # Test the API key by creating a StoryEngine instance with it
        test_engine = StoryEngine(api_key=gemini_key)
        console.print("[green]✅ Gemini API connection successful[/green]")
    except Exception as e:
        console.print(f"[red]❌ Gemini API test failed: {e}[/red]")
        console.print("[yellow]Please check your API key and try again.[/yellow]")
        sys.exit(1)

    # MongoDB Configuration
    console.print("\n[yellow]🗄️  MongoDB Configuration[/yellow]")

    # First, ask for MongoDB instance/connection URL
    mongodb_url = os.getenv("MONGODB_URL", "").strip()
    if not mongodb_url:
        console.print("Please provide your MongoDB connection details.")
        mongodb_url = Prompt.ask("Enter MongoDB connection URL", default="mongodb://localhost:27017/")
    else:
        console.print(f"[green]✅ MongoDB URL found: {mongodb_url}[/green]")

    # Then ask if they want to run it locally
    if "localhost" in mongodb_url or "127.0.0.1" in mongodb_url:
        if Confirm.ask("Would you like to run MongoDB locally with Docker?", default=True):
            console.print("[yellow]🐳 Setting up local MongoDB with Docker...[/yellow]")

            # Check if Docker is available
            import subprocess
            try:
                result = subprocess.run(["docker", "--version"], capture_output=True, text=True, check=True)
                console.print("[green]✅ Docker found[/green]")
            except (subprocess.CalledProcessError, FileNotFoundError):
                console.print("[red]❌ Docker not found. Please install Docker or provide a different MongoDB URL.[/red]")
                mongodb_url = Prompt.ask("Enter MongoDB connection URL", default="mongodb://localhost:27017/")
            else:
                # Start MongoDB with Docker
                try:
                    # Check if container already exists
                    result = subprocess.run(["docker", "ps", "-a", "--filter", "name=git-whisperer-mongo", "--format", "{{.Names}}"],
                                          capture_output=True, text=True)

                    if "git-whisperer-mongo" in result.stdout:
                        console.print("[yellow]📦 MongoDB container exists, starting it...[/yellow]")
                        subprocess.run(["docker", "start", "git-whisperer-mongo"], check=True)
                    else:
                        console.print("[yellow]📦 Creating and starting MongoDB container...[/yellow]")
                        subprocess.run([
                            "docker", "run", "-d",
                            "--name", "git-whisperer-mongo",
                            "-p", "27017:27017",
                            "mongo:7.0"
                        ], check=True)

                    console.print("[green]✅ Local MongoDB started successfully[/green]")
                    mongodb_url = "mongodb://localhost:27017/"

                    # Wait a moment for MongoDB to be ready
                    import time
                    console.print("[yellow]⏳ Waiting for MongoDB to be ready...[/yellow]")
                    time.sleep(3)

                except subprocess.CalledProcessError as e:
                    console.print(f"[red]❌ Failed to start MongoDB: {e}[/red]")
                    mongodb_url = Prompt.ask("Enter MongoDB connection URL", default="mongodb://localhost:27017/")
        else:
            console.print("[yellow]Using provided MongoDB URL...[/yellow]")

    # Test MongoDB connection
    console.print(f"\n[yellow]🔍 Testing MongoDB connection: {mongodb_url}[/yellow]")
    try:
        # Temporarily set MongoDB URL for testing
        original_url = os.getenv("MONGODB_URL")
        os.environ["MONGODB_URL"] = mongodb_url

        test_store = MemoryStore()
        console.print("[green]✅ MongoDB connection successful[/green]")
        config["MONGODB_URL"] = mongodb_url

    except Exception as e:
        console.print(f"[red]❌ MongoDB connection failed: {e}[/red]")
        console.print("[yellow]Please check your MongoDB configuration and try again.[/yellow]")
        sys.exit(1)

    # Save configuration to .env file
    if config:
        save_configuration(config)
        console.print(f"\n[green]💾 Configuration saved to .env file[/green]")

    # Reload environment and reconfigure logging
    load_dotenv()
    EnvVars.reload()
    # Reconfigure logging with proper log level
    log_level = getattr(logging, os.getenv("LOG_LEVEL", "INFO").upper(), logging.INFO)
    logging.getLogger().setLevel(log_level)

    console.print("\n[green]🎉 Setup complete! You're ready to analyze repositories.[/green]\n")

def save_configuration(config):
    """
    Save configuration to .env file.
    """
    env_file = Path(".env")

    # Read existing .env content
    existing_content = {}
    if env_file.exists():
        with open(env_file, 'r') as f:
            for line in f:
                line = line.strip()
                if line and not line.startswith('#') and '=' in line:
                    key, value = line.split('=', 1)
                    existing_content[key] = value

    # Update with new configuration
    existing_content.update(config)

    # Write back to .env file
    with open(env_file, 'w') as f:
        f.write("# Git Whisperer Configuration\n")
        f.write("# Auto-generated by setup process\n\n")

        for key, value in existing_content.items():
            if key == "GEMINI_API_KEY":
                f.write(f"{key}={value}\n")
            else:
                f.write(f"{key}={value}\n")

        f.write("\n")

def analyze_repository(repo_path: str) -> dict:
    """
    Analyze a git repository and return structured data.

    Args:
        repo_path (str): Path to the git repository

    Returns:
        dict: Analysis results containing commits and metadata
    """
    logger.info(f"[bold blue]🔍 Starting analysis of repository:[/bold blue] {repo_path}")

    # Validate repository
    if not os.path.exists(repo_path):
        raise ValueError(f"Repository path '{repo_path}' does not exist.")

    if not os.path.exists(os.path.join(repo_path, '.git')):
        raise ValueError(f"'{repo_path}' is not a git repository.")

    # Get git log
    logger.info("📚 Fetching git history...")
    log_output = get_git_log(repo_path)

    # Parse commits
    commits = parse_git_log(log_output)
    logger.info(f"[green]✅ Found {len(commits)} commits.[/green]")

    return {
        'repo_path': repo_path,
        'commits': commits,
        'commit_count': len(commits)
    }

def store_analysis(analysis_data: dict) -> None:
    """
    Store the analysis data in the configured database.

    Args:
        analysis_data (dict): Analysis results from analyze_repository
    """
    logger.info("💾 Storing analysis in database...")

    with console.status("[bold green]Connecting to database...") as status:
        try:
            store = MemoryStore()
            status.update("[bold green]Saving commits...")
            store.save_commits(analysis_data['commits'])
            logger.info("[green]✅ Successfully stored commits in database.[/green]")
        except Exception as e:
            logger.error(f"[red]❌ Failed to store analysis: {e}[/red]")
            raise

def generate_story(analysis_data: dict) -> str:
    """
    Generate a human-readable story from the analysis data.

    Args:
        analysis_data (dict): Analysis results from analyze_repository

    Returns:
        str: Generated project story
    """
    logger.info("🤖 Generating project story with AI...")

    with console.status("[bold blue]AI is crafting your story...") as status:
        try:
            engine = StoryEngine()
            status.update("[bold blue]Analyzing commit patterns...")
            summary = engine.generate_project_summary(analysis_data['commits'])
            logger.info("[green]✅ Successfully generated project story![/green]")
            return summary
        except Exception as e:
            logger.error(f"[red]❌ Failed to generate story: {e}[/red]")
            raise

def print_results(analysis_data: dict, story: str) -> None:
    """
    Print the analysis results and story in a formatted way.

    Args:
        analysis_data (dict): Analysis results
        story (str): Generated story
    """
    # Create a beautiful header
    header_text = Text("GIT WHISPERER ANALYSIS RESULTS", style="bold magenta")
    header_panel = Panel(header_text, border_style="magenta")

    console.print(header_panel)

    # Repository info
    repo_info = f"""
[bold cyan]📁 Repository:[/bold cyan] {analysis_data['repo_path']}
[bold cyan]📊 Commits analyzed:[/bold cyan] {analysis_data['commit_count']}
[bold cyan]🤖 AI Model:[/bold cyan] Gemini 2.5 Flash
[bold cyan]💾 Database:[/bold cyan] {EnvVars.MONGODB_DB}
"""

    console.print(Panel(repo_info.strip(), title="[bold green]Analysis Summary[/bold green]", border_style="green"))

    # Project story in a beautiful panel
    story_panel = Panel(
        story,
        title="[bold yellow]📖 Project Story[/bold yellow]",
        border_style="yellow",
        padding=(1, 2)
    )
    console.print(story_panel)

    # Success message
    success_text = Text("✨ Analysis complete! Your project story is ready.", style="bold green")
    console.print(Panel(success_text, border_style="green"))

def main():
    """Main entry point for Git Whisperer."""
    # Check for setup flag
    if len(sys.argv) >= 2 and sys.argv[1] == "--setup":
        setup_configuration()
        console.print("\n[green]🎉 Setup complete! You can now analyze repositories.[/green]")
        sys.exit(0)

    if len(sys.argv) < 2:
        # Beautiful welcome message
        welcome_text = Text("Git Whisperer", style="bold magenta")
        welcome_panel = Panel(
            welcome_text,
            title="[bold blue]🎭 Welcome to[/bold blue]",
            border_style="blue"
        )
        console.print(welcome_panel)

        help_text = """
[bold cyan]Turn commit history into human stories[/bold cyan]

[dim]Analyze your Git repository and generate compelling narratives about your project's evolution using AI.[/dim]

[bold green]Usage:[/bold green]
  python main.py <path-to-git-repo>
  python main.py --setup              [dim]# Run interactive setup[/dim]

[bold green]Examples:[/bold green]
  python main.py /path/to/your/project
  python main.py .                    [dim]# Analyze current directory[/dim]

[bold yellow]Setup:[/bold yellow]
  • First run will guide you through configuration
  • Gemini API key and MongoDB setup handled automatically
  • Configuration saved for future runs

[dim]Get your API key from: https://makersuite.google.com/app/apikey[/dim]
"""
        console.print(Panel(help_text.strip(), border_style="cyan"))
        sys.exit(1)

    repo_path = sys.argv[1]

    # Run interactive setup if configuration is missing
    try:
        # Try to validate existing configuration
        EnvVars.validate()
        # Test MongoDB connection
        test_store = MemoryStore()
    except Exception:
        # Configuration is missing or invalid, run setup
        setup_configuration()

    try:
        # Analyze the repository
        analysis_data = analyze_repository(repo_path)

        # Store the analysis
        store_analysis(analysis_data)

        # Generate the story
        story = generate_story(analysis_data)

        # Print results
        print_results(analysis_data, story)

    except KeyboardInterrupt:
        logger.info("[yellow]⚠️  Analysis interrupted by user.[/yellow]")
        console.print("[yellow]Analysis cancelled.[/yellow]")
        sys.exit(1)
    except Exception as e:
        error_text = f"[red]❌ Error: {str(e)}[/red]"
        console.print(Panel(error_text, border_style="red", title="[bold red]Error[/bold red]"))

        # Troubleshooting panel
        troubleshooting = """
[yellow]🔧 Troubleshooting:[/yellow]
• Make sure the path is a valid git repository
• Check that GEMINI_API_KEY is set in your .env file
• Ensure MongoDB is running (if using local MongoDB)
• Verify all dependencies are installed
"""
        console.print(Panel(troubleshooting.strip(), border_style="yellow", title="[bold yellow]Help[/bold yellow]"))
        sys.exit(1)

if __name__ == "__main__":
    main()