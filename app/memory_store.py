from pymongo import MongoClient
from .config.envVars import EnvVars

class MemoryStore:
    def __init__(self, db_name=None, collection_name=None):
        """
        Initializes the connection to the MongoDB database and collection.
        Uses configuration from EnvVars if not provided.
        """
        self.db_name = db_name or EnvVars.MONGODB_DB
        self.collection_name = collection_name or EnvVars.MONGODB_COLLECTION

        try:
            mongodb_url = EnvVars.get_mongodb_connection_string()
            self.client = MongoClient(mongodb_url)
            self.db = self.client[self.db_name]
            self.collection = self.db[self.collection_name]
            # Test the connection
            self.client.server_info()
            print("Successfully connected to MongoDB.")
        except Exception as e:
            raise Exception(f"Could not connect to MongoDB at {mongodb_url}: {e}")

    def save_commits(self, commits):
        """
        Saves a list of commit dictionaries to the database.
        This will overwrite existing commits with the same hash.
        """
        if not commits:
            return

        for commit in commits:
            # Use the commit hash as the unique identifier (_id)
            # This prevents duplicate entries for the same commit
            commit['_id'] = commit['hash']
            self.collection.replace_one({'_id': commit['hash']}, commit, upsert=True)
        
        print(f"Saved {len(commits)} commits to the database.")

    def get_all_commits(self):
        """
        Retrieves all commits from the database.
        """
        return list(self.collection.find())

if __name__ == '__main__':
    # Example usage
    try:
        # Initialize the memory store
        store = MemoryStore()

        # Example commit data
        example_commits = [
            {
                "hash": "abcdef1",
                "message": "feat: Initial commit",
                "files_changed": [
                    {"file_path": "README.md", "summary": "1 +"}
                ]
            },
            {
                "hash": "abcdef2",
                "message": "fix: A bug fix",
                "files_changed": [
                    {"file_path": "main.py", "summary": "2 +-"}
                ]
            }
        ]

        # Save the example commits
        store.save_commits(example_commits)

        # Retrieve and print all commits
        all_commits = store.get_all_commits()
        print("\n--- All Commits in DB ---")
        import json
        print(json.dumps(all_commits, indent=2))

    except Exception as e:
        print(f"An error occurred: {e}")
