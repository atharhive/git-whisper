from src.history_analyzer import get_git_log, parse_git_log
from src.memory_store import MemoryStore
from src.story_engine import StoryEngine
import json
import os

def main():
    """
    Main function to analyze git history, store it in MongoDB, and generate narratives using Gemini.
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
        print("\n--- Generating Narratives with Gemini ---")
        if not os.getenv("GEMINI_API_KEY"):
            print("Error: GEMINI_API_KEY environment variable not set. Please set it to use Gemini API.")
            return

        story_engine = StoryEngine()

        print("\n--- Project Summary ---")
        summary = story_engine.generate_project_summary(all_commits)
        print(summary)

        print("\n--- Changelog ---")
        changelog = story_engine.generate_changelog(all_commits)
        print(changelog)

        print("\n--- Demo Script ---")
        demo_script = story_engine.generate_demo_script(all_commits)
        print(demo_script)

    except ValueError as e:
        print(f"Configuration Error: {e}")
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    main()