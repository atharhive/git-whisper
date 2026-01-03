# Git Whisperer - Docker Setup

This directory contains Docker configuration files for running Git Whisperer in containers.

## Quick Start

### Using Docker Compose (Recommended)

1. **Clone the repository:**
   ```bash
   git clone https://github.com/atharhive/git-whisperer.git
   cd git-whisperer
   ```

2. **Set up environment variables:**
   ```bash
   cp .env.example .env
   # Edit .env with your API keys
   ```

3. **Start the services:**
   ```bash
   docker-compose -f docker/compose.yaml up -d
   ```

4. **Run Git Whisperer:**
   ```bash
   docker-compose -f docker/compose.yaml exec app whisper --help
   ```

### Using Individual Services

#### Full Stack (App + MongoDB)
```bash
# Start both services
docker-compose -f docker/compose.yaml up -d

# Run commands
docker-compose -f docker/compose.yaml exec app whisper add <repo-url>
docker-compose -f docker/compose.yaml exec app whisper summary
```

#### MongoDB Only
```bash
# Start just MongoDB
docker-compose -f docker/mongo/compose.yaml up -d

# Connect to MongoDB from your local Git Whisperer installation
```

#### App Only (requires external MongoDB)
```bash
# Build and run the app container
docker build -f docker/Dockerfile -t git-whisperer .
docker run -it --rm \
  -e GEMINI_API_KEY=your_key \
  -e MONGODB_URL=mongodb://host:port \
  git-whisperer whisper --help
```

## Configuration

### Environment Variables

Create a `.env` file in the project root:

```env
# Required: Google Gemini API Key
GEMINI_API_KEY=your_gemini_api_key_here

# MongoDB Configuration
MONGODB_URL=mongodb://mongodb:27017/
MONGODB_DB=git_whisperer_db
MONGODB_COLLECTION=project_history
LOG_LEVEL=INFO
```

### Docker Compose Overrides

For development, use `docker-compose.override.yaml`:

```bash
docker-compose -f docker/compose.yaml -f docker-compose.override.yaml up -d
```

This provides:
- Volume mounts for live code reloading
- Debug logging
- Development environment settings

## Development Workflow

1. **Start services:**
   ```bash
   docker-compose -f docker/compose.yaml -f docker-compose.override.yaml up -d
   ```

2. **Run setup:**
   ```bash
   docker-compose -f docker/compose.yaml exec app whisper setup
   ```

3. **Analyze repositories:**
   ```bash
   docker-compose -f docker/compose.yaml exec app whisper add <repo-url>
   docker-compose -f docker/compose.yaml exec app whisper summary
   ```

4. **View logs:**
   ```bash
   docker-compose -f docker/compose.yaml logs -f app
   ```

## File Structure

```
docker/
├── compose.yaml          # Full stack (app + mongodb)
├── core/compose.yaml     # App only (requires external mongodb)
├── mongo/compose.yaml    # MongoDB only
├── Dockerfile           # Rust application build
└── README.md           # This file
```

## Troubleshooting

### Build Issues
- Ensure you have Docker Desktop installed
- Check that Rust 1.83+ is available in the build image
- Verify all dependencies are properly installed

### Runtime Issues
- Check environment variables are set correctly
- Ensure MongoDB is accessible from the app container
- Verify API keys are valid

### Database Connection
- MongoDB runs on port 27017 by default
- Use `mongodb://mongodb:27017/` for inter-container communication
- Use `mongodb://localhost:27017/` for external connections

## Production Deployment

For production, consider:
- Using external MongoDB (MongoDB Atlas)
- Setting up proper secrets management
- Configuring health checks and monitoring
- Using multi-stage builds for smaller images