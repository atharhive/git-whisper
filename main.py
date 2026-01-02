from src.history_analyzer import get_git_log, parse_git_log
from src.memory_store import MemoryStore
from src.story_engine import StoryEngine
import json
import os
import argparse

def analyze_repository(api_key, generate_summary=False, generate_changelog=False, generate_demo_script=False):
    """
    Analyzes git history, stores it, and generates narratives.
    """
    try:
        # 1. Analyze git history
        print("--- Analyzing Git History ---")
        log_output = get_git_log(".")
        parsed_log = parse_git_log(log_output)
        print(f"Found {len(parsed_log)} commits.")

        # 2. Store commits in MongoDB
        print("\n--- Storing Commits in MongoDB ---")
        memory_store = MemoryStore()
        memory_store.save_commits(parsed_log)

        # 3. Retrieve all commits from the database
        all_commits = memory_store.get_all_commits()
        print(f"Retrieved {len(all_commits)} commits from DB.")

        # 4. Generate narratives using Gemini
        if generate_summary or generate_changelog or generate_demo_script:
            print("\n--- Generating Narratives with Gemini ---")
            story_engine = StoryEngine(api_key=api_key)

            if generate_summary:
                print("\n--- Project Summary ---")
                summary = story_engine.generate_project_summary(all_commits)
                print(summary)

            if generate_changelog:
                print("\n--- Changelog ---")
                changelog = story_engine.generate_changelog(all_commits)
                print(changelog)

            if generate_demo_script:
                print("\n--- Demo Script ---")
                demo_script = story_engine.generate_demo_script(all_commits)
                print(demo_script)

    except ValueError as e:
        print(f"Configuration Error: {e}")
    except Exception as e:
        print(f"An error occurred: {e}")

def main():
    """
    Main entry point for the Git Whisperer CLI.
    """
    parser = argparse.ArgumentParser(description="Git Whisperer: Turn your git history into a human story.")
    
    subparsers = parser.add_subparsers(dest="command")

    # Analyze command
    analyze_parser = subparsers.add_parser("analyze", help="Analyze a git repository.")
    analyze_parser.add_argument("--summary", action="store_true", help="Generate a project summary.")
    analyze_parser.add_argument("--changelog", action="store_true", help="Generate a changelog.")
    analyze_parser.add_argument("--demo-script", action="store_true", help="Generate a demo script.")
    analyze_parser.add_argument("--all", action="store_true", help="Generate all narrative outputs.")

    # Models command
    models_parser = subparsers.add_parser("models", help="List available Gemini models.")
    
    args = parser.parse_args()

    api_key = os.getenv("GEMINI_API_KEY")
    if not api_key:
        print("Error: GEMINI_API_KEY environment variable not set. Please set it to use Gemini API.")
        return

    if args.command == "analyze":
        if args.all:
            analyze_repository(api_key, generate_summary=True, generate_changelog=True, generate_demo_script=True)
        elif args.summary or args.changelog or args.demo_script:
            analyze_repository(api_key, generate_summary=args.summary, generate_changelog=args.changelog, generate_demo_script=args.demo_script)
        else:
            # If no specific output is requested, just analyze and store
            analyze_repository(api_key)
            print("\nAnalysis complete. Commits are stored in the memory store.")
            print("To generate narratives, use --summary, --changelog, --demo-script, or --all.")
    
    elif args.command == "models":
        story_engine = StoryEngine(api_key=api_key)
        story_engine.list_available_models()


if __name__ == "__main__":
    main()