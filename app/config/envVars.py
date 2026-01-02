import os
from dotenv import load_dotenv

class EnvVars:
    """
    Environment variables configuration for Git Whisperer.
    """

    # Class variables that get updated dynamically
    GEMINI_API_KEY: str = ""
    MONGODB_URL: str = "mongodb://localhost:27017/"
    MONGODB_DB: str = "git_whisperer_db"
    MONGODB_COLLECTION: str = "project_history"
    MONGODB_USERNAME: str = ""
    MONGODB_PASSWORD: str = ""
    LOG_LEVEL: str = "INFO"

    @classmethod
    def reload(cls):
        """Reload environment variables from .env file and update class attributes"""
        load_dotenv()
        cls.GEMINI_API_KEY = os.getenv("GEMINI_API_KEY", "")
        cls.MONGODB_URL = os.getenv("MONGODB_URL", "mongodb://localhost:27017/")
        cls.MONGODB_DB = os.getenv("MONGODB_DB", "git_whisperer_db")
        cls.MONGODB_COLLECTION = os.getenv("MONGODB_COLLECTION", "project_history")
        cls.MONGODB_USERNAME = os.getenv("MONGODB_USERNAME", "")
        cls.MONGODB_PASSWORD = os.getenv("MONGODB_PASSWORD", "")
        cls.LOG_LEVEL = os.getenv("LOG_LEVEL", "INFO")

    @classmethod
    def validate(cls) -> None:
        """
        Validate that all required environment variables are set.
        Raises ValueError if any required variable is missing.
        """
        missing_vars = []

        if not cls.GEMINI_API_KEY:
            missing_vars.append("GEMINI_API_KEY")

        if missing_vars:
            raise ValueError(
                f"Missing required environment variables: {', '.join(missing_vars)}\n"
                "Please set these in your .env file or environment.\n"
                "See .env.example for required variables."
            )

    @classmethod
    def get_mongodb_connection_string(cls) -> str:
        """
        Build MongoDB connection string with authentication if provided.
        """
        base_url = cls.MONGODB_URL

        # If username and password are provided, insert them into the URL
        if cls.MONGODB_USERNAME and cls.MONGODB_PASSWORD:
            # Handle different URL formats
            if "://" in base_url:
                protocol, rest = base_url.split("://", 1)
                return f"{protocol}://{cls.MONGODB_USERNAME}:{cls.MONGODB_PASSWORD}@{rest}"
            else:
                return f"mongodb://{cls.MONGODB_USERNAME}:{cls.MONGODB_PASSWORD}@{base_url}"

        return base_url

# Validate configuration on import
# EnvVars.validate()  # Commented out - validation now handled in main.py