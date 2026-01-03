# Git Whisperer Setup Guide

## Prerequisites

You need two things:
1. **Google Gemini API Key** (free)
2. **MongoDB** (local or cloud)

---

## 1. Get Your Gemini API Key

### Steps:
1. Go to [Google AI Studio](https://makersuite.google.com/app/apikey)
2. Sign in with your Google account
3. Click "Create API Key"
4. Copy the key (starts with `AIza...`)

**Note:** The free tier includes:
- 15 requests per minute
- 1,500 requests per day
- More than enough for analyzing repositories

---

## 2. Choose Your MongoDB Setup

### Option A: Local MongoDB with Docker (Easiest)

**Requirements:** Docker installed

The setup wizard will automatically:
- Pull the MongoDB Docker image
- Start a container named `git-whisperer-mongo`
- Expose it on `localhost:27017`

Just run `git-whisperer setup` and choose option 1.

---

### Option B: MongoDB Atlas (Cloud - Free Tier)

**Steps:**

1. **Create Account**
   - Go to [MongoDB Atlas](https://www.mongodb.com/cloud/atlas/register)
   - Sign up for free

2. **Create Cluster**
   - Click "Build a Database"
   - Choose "M0 Free" tier
   - Select a cloud provider and region
   - Click "Create Cluster"

3. **Create Database User**
   - Go to "Database Access" in left sidebar
   - Click "Add New Database User"
   - Choose "Password" authentication
   - Set username and password (save these!)
   - Set role to "Read and write to any database"
   - Click "Add User"

4. **Whitelist Your IP**
   - Go to "Network Access" in left sidebar
   - Click "Add IP Address"
   - Click "Allow Access from Anywhere" (or add your specific IP)
   - Click "Confirm"

5. **Get Connection String**
   - Go back to "Database" (left sidebar)
   - Click "Connect" on your cluster
   - Choose "Connect your application"
   - Copy the connection string (looks like):
     ```
     mongodb+srv://username:password@cluster0.xxxxx.mongodb.net/?retryWrites=true&w=majority
     ```
   - Replace `<password>` with your actual password
   - Replace `username` with your actual username

6. **Use in Setup**
   - Run `git-whisperer setup`
   - Choose option 2 (MongoDB Atlas)
   - Paste your connection string

**Free Tier Limits:**
- 512 MB storage
- Shared RAM
- Perfect for this use case!

---

### Option C: Custom MongoDB

If you have MongoDB installed elsewhere:

**Local MongoDB (no Docker):**
```
mongodb://localhost:27017/
```

**MongoDB with Authentication:**
```
mongodb://username:password@host:port/
```

**MongoDB Replica Set:**
```
mongodb://host1:port1,host2:port2,host3:port3/?replicaSet=myReplicaSet
```

---

## 3. Run Setup

```bash
# Build the project
cargo build --release

# Run setup wizard
./target/release/git-whisperer setup
```

The wizard will:
1. ✅ Ask for your Gemini API key
2. ✅ Test the API connection
3. ✅ Let you choose MongoDB setup
4. ✅ Test the database connection
5. ✅ Save everything to `.env`

---

## 4. Manual Configuration (Alternative)

If you prefer to skip the wizard:

1. Copy the example config:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env`:
   ```env
   # Your actual Gemini API key
   GEMINI_API_KEY=AIzaSyC_your_actual_key_here
   
   # For MongoDB Atlas:
   MONGODB_URL=mongodb+srv://user:pass@cluster.mongodb.net/?retryWrites=true&w=majority
   
   # Or for local:
   MONGODB_URL=mongodb://localhost:27017/
   
   MONGODB_DB=git_whisperer_db
   MONGODB_COLLECTION=project_history
   LOG_LEVEL=INFO
   ```

3. Test it:
   ```bash
   git-whisperer /path/to/repo
   ```

---

## Troubleshooting

### "Failed to connect to Gemini API"
- Check your API key is correct
- Verify you have internet connection
- Check if you've exceeded rate limits (wait a minute)

### "Failed to connect to MongoDB"
- **Local:** Make sure Docker is running and container is started
- **Atlas:** 
  - Verify your IP is whitelisted
  - Check username/password in connection string
  - Ensure cluster is running (not paused)
- Test connection with `mongosh` or MongoDB Compass

### "Docker not found"
- Install Docker Desktop: https://www.docker.com/products/docker-desktop
- Or choose MongoDB Atlas instead

---

## Verify Setup

After setup, verify everything works:

```bash
# Analyze a repository
git-whisperer /path/to/your/repo

# Or analyze current directory
cd your-repo
git-whisperer .
```

You should see:
- ✅ Git history fetched
- ✅ Commits stored in database
- ✅ AI-generated story

---

## Security Notes

- **Never commit `.env` file** (it's in `.gitignore`)
- **Keep your API keys private**
- **For MongoDB Atlas:** Use strong passwords
- **Connection strings contain credentials** - treat them like passwords

---

## Cost

Both services are **completely free** for this use case:

- **Gemini API:** Free tier is generous
- **MongoDB Atlas:** M0 tier is free forever

You won't need to pay anything unless you're analyzing thousands of repos per day.
