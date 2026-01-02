import subprocess
import re

def get_git_log(repo_path):
    """
    Executes 'git log --oneline --stat' in the given repository path.

    Args:
        repo_path (str): The path to the git repository.

    Returns:
        str: The output of the git log command.
    """
    command = ["git", "log", "--oneline", "--stat"]
    try:
        result = subprocess.run(
            command,
            cwd=repo_path,
            capture_output=True,
            text=True,
            check=True
        )
        return result.stdout
    except FileNotFoundError:
        raise Exception("Git is not installed or not in your PATH.")
    except subprocess.CalledProcessError as e:
        raise Exception(f"Error running git log: {e.stderr}")

def parse_git_log(log_output):
    """
    Parses the output of 'git log --oneline --stat' into a structured format.

    Args:
        log_output (str): The string output from the git log command.

    Returns:
        list: A list of dictionaries, where each dictionary represents a commit.
    """
    commits = []
    # Split by the commit separator which is a newline followed by a commit hash
    commit_blocks = re.split(r'\n(?=[0-9a-f]{7,}\s)', log_output.strip())

    for block in commit_blocks:
        if not block.strip():
            continue

        lines = block.strip().split('\n')
        commit_line = lines[0]
        
        parts = commit_line.split(' ', 1)
        commit_hash = parts[0]
        message = parts[1]
        
        commit = {
            "hash": commit_hash,
            "message": message,
            "files_changed": []
        }

        # The rest of the lines are file stats
        for line in lines[1:]:
            line = line.strip()
            if '|' in line and 'changed' not in line:
                file_parts = line.split('|')
                file_path = file_parts[0].strip()
                summary = file_parts[1].strip()
                commit["files_changed"].append({
                    "file_path": file_path,
                    "summary": summary
                })

        commits.append(commit)

    return commits

if __name__ == '__main__':
    # Example usage:
    # This assumes you are running this from a directory that contains a .git folder
    # or you provide a valid path to a git repository.
    try:
        log = get_git_log(".")
        parsed_log = parse_git_log(log)
        import json
        print(json.dumps(parsed_log, indent=2))
    except Exception as e:
        print(f"An error occurred: {e}")
