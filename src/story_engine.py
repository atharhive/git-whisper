import google.generativeai as genai
import os

class StoryEngine:
    def __init__(self, api_key=None):
        """
        Initializes the Gemini API client.
        If api_key is provided, uses it. Otherwise, assumes GEMINI_API_KEY is set as an environment variable.
        """
        if api_key:
            self._api_key = api_key
        else:
            self._api_key = os.getenv("GEMINI_API_KEY")
        
        if not self._api_key:
            raise ValueError("GEMINI_API_KEY environment variable not set and no API key provided.")
        
        genai.configure(api_key=self._api_key)
        self.model = genai.GenerativeModel('gemini-pro')

    def generate_project_summary(self, commits):
        """
        Generates a human-readable project summary from commit data using Gemini.

        Args:
            commits (list): A list of dictionaries, where each dictionary represents a commit.

        Returns:
            str: The generated project summary.
        """
        if not commits:
            return "No commits provided to generate a summary."

        commit_history_text = "Commit History:\n"
        for commit in commits:
            commit_history_text += f"- {commit['hash']}: {commit['message']}\n"
            for file_change in commit['files_changed']:
                commit_history_text += f"  - {file_change['file_path']} ({file_change['summary']})\n"
        
        prompt = f"""Given the following git commit history, generate a concise, plain-English project summary. Explain what problem the project tried to solve, how it evolved, and what actually matters. Focus on intent and evolution, not implementation details. The summary should be similar to the example:

"This project evolved from an initial scaffold into a functional application with authentication, performance optimizations, and a refined developer experience."

{commit_history_text}"""
        
        try:
            response = self.model.generate_content(prompt)
            return response.text
        except Exception as e:
            return f"Error generating summary: {e}"

    def generate_changelog(self, commits):
        """
        Generates a clean, readable CHANGELOG draft from commit data using Gemini.

        Args:
            commits (list): A list of dictionaries, where each dictionary represents a commit.

        Returns:
            str: The generated CHANGELOG draft.
        """
        if not commits:
            return "No commits provided to generate a changelog."

        commit_history_text = "Commit History:\n"
        for commit in commits:
            commit_history_text += f"- {commit['hash']}: {commit['message']}\n"
            for file_change in commit['files_changed']:
                commit_history_text += f"  - {file_change['file_path']} ({file_change['summary']})\n"
        
        prompt = f"""Given the following git commit history, generate a clean, readable CHANGELOG draft. Group related changes and highlight key features, fixes, and improvements. The changelog should be similar to the example:

"Added JWT-based authentication and refactored middleware to support scaling."

{commit_history_text}"""
        
        try:
            response = self.model.generate_content(prompt)
            return response.text
        except Exception as e:
            return f"Error generating changelog: {e}"

    def generate_demo_script(self, commits):
        """
        Generates a 60-90 second demo narration script from commit data using Gemini.

        Args:
            commits (list): A list of dictionaries, where each dictionary represents a commit.

        Returns:
            str: The generated demo narration script.
        """
        if not commits:
            return "No commits provided to generate a demo script."

        commit_history_text = "Commit History:\n"
        for commit in commits:
            commit_history_text += f"- {commit['hash']}: {commit['message']}\n"
            for file_change in commit['files_changed']:
                commit_history_text += f"  - {file_change['file_path']} ({file_change['summary']})\n"
        
        prompt = f"""Given the following git commit history, generate a 60-90 second demo narration script. The script should tell the story of the project's evolution, highlighting key milestones and decisions. Focus on the 'why' behind the changes, not just the 'what'. The script should be similar to the example:

"I started with a basic scaffold, then focused on user authentication. After hitting performance issues, I refactored the core logic, which shaped the final architecture."

{commit_history_text}"""
        
        try:
            response = self.model.generate_content(prompt)
            return response.text
        except Exception as e:
            return f"Error generating demo script: {e}"

if __name__ == '__main__':
    # Example usage:
    # Requires GEMINI_API_KEY to be set as an environment variable
    # For testing, you might need some dummy commit data.
    example_commits = [
        {
            "hash": "abcdef1",
            "message": "feat: Implement user authentication",
            "files_changed": [{"file_path": "auth.py", "summary": "100 ++"}]
        },
        {
            "hash": "abcdef2",
            "message": "fix: A bug fix",
            "files_changed": [{"file_path": "main.py", "summary": "2 +-"}]
        },
        {
            "hash": "abcdef3",
            "message": "refactor: Optimize database queries for performance",
            "files_changed": [{"file_path": "db.py", "summary": "50 +--"}]
        }
    ]
    
    try:
        engine = StoryEngine()
        print("\n--- Project Summary ---")
        summary = engine.generate_project_summary(example_commits)
        print(summary)

        print("\n--- Changelog ---")
        changelog = engine.generate_changelog(example_commits)
        print(changelog)

        print("\n--- Demo Script ---")
        demo_script = engine.generate_demo_script(example_commits)
        print(demo_script)

    except ValueError as e:
        print(f"Configuration Error: {e}")
    except Exception as e:
        print(f"An error occurred: {e}")