use anyhow::Result;
use colored::*;
use dialoguer::{Input, Confirm, Password};
use std::io::Write;

use crate::config::Config;
use crate::gemini::GeminiClient;
use crate::storage::MongoStore;

pub async fn run_setup() -> Result<()> {
    // Welcome banner
    println!("{}", "╔═══════════════════════════════════════╗".bright_blue());
    println!("{}", "║           🎭 Git Whisperer            ║".bright_blue().bold());
    println!("{}", "║           Setup Wizard                ║".bright_blue());
    println!("{}", "╚═══════════════════════════════════════╝".bright_blue());
    println!();

    let mut config = Config::load_or_default()?;

    // Step 1: Gemini API Key
    println!("{}", "┌─ Step 1: Gemini API Configuration ──────────────────────┐".bright_cyan());
    println!("{}", "│                                                         │".bright_cyan());

    if config.gemini_api_key.is_empty() {
        println!("{}", "│ 🤖 Gemini API Key Required                             │".bright_cyan());
        println!("{}", "│                                                         │".bright_cyan());
        println!("{}", "│ Get your API key from:                                  │".bright_cyan());
        println!("{}", "│ 🔗 https://makersuite.google.com/app/apikey             │".bright_cyan());
        println!("{}", "│                                                         │".bright_cyan());
        println!("{}", "└─────────────────────────────────────────────────────────┘".bright_cyan());
        println!();

        let api_key: String = Password::new()
            .with_prompt("🔑 Enter your Gemini API key")
            .interact()?;

        if api_key.trim().is_empty() {
            println!("{}", "❌ API key cannot be empty. Setup cancelled.".red());
            anyhow::bail!("Gemini API key is required");
        }

        config.gemini_api_key = api_key.trim().to_string();
        println!("{}", "✅ API key configured".green());
    } else {
        println!("{}", "│ ✅ Gemini API key already configured                   │".bright_cyan());
        println!("{}", "└─────────────────────────────────────────────────────────┘".bright_cyan());
    }

    // Step 2: Test Gemini API
    println!();
    println!("{}", "┌─ Step 2: Testing Gemini API Connection ─────────────────┐".bright_yellow());
    println!("{}", "│                                                         │".bright_yellow());
    println!("{}", "│ 🔍 Verifying API key...                                │".bright_yellow());
    println!("{}", "│                                                         │".bright_yellow());
    println!("{}", "└─────────────────────────────────────────────────────────┘".bright_yellow());

    match GeminiClient::new(&config.gemini_api_key) {
        Ok(client) => {
            match client.test_connection().await {
                Ok(_) => {
                    println!("{}", "✅ Gemini API connection successful".green().bold());
                }
                Err(e) => {
                    println!("{}", format!("❌ Gemini API test failed: {}", e).red());
                    println!("{}", "💡 Please check your API key and try again.".yellow());
                    anyhow::bail!("Gemini API connection failed");
                }
            }
        }
        Err(e) => {
            println!("{}", format!("❌ Failed to initialize Gemini client: {}", e).red());
            anyhow::bail!("Gemini client initialization failed");
        }
    }

    // Step 3: MongoDB Configuration
    println!();
    println!("{}", "┌─ Step 3: MongoDB Database Configuration ────────────────┐".bright_magenta());
    println!("{}", "│                                                         │".bright_magenta());

    if config.mongodb_url.is_empty() || config.mongodb_url == "mongodb://localhost:27017/" {
        // Ask if user already has a MongoDB connection
        let has_existing_connection = Confirm::new()
            .with_prompt("🗄️  Do you already have a MongoDB connection (Atlas, local, etc.)?")
            .default(false)
            .interact()?;

        if has_existing_connection {
            println!();
            println!("{}", "┌─ Existing MongoDB Connection ───────────────────────────┐".bright_blue());
            println!("{}", "│                                                         │".bright_blue());
            println!("{}", "│ 🔗 Please provide your MongoDB connection string       │".bright_blue());
            println!("{}", "│                                                         │".bright_blue());
            println!("{}", "└─────────────────────────────────────────────────────────┘".bright_blue());
            println!();

            let connection_string: String = Input::new()
                .with_prompt("🔗 Enter your MongoDB connection string")
                .interact_text()?;

            if connection_string.trim().is_empty() {
                println!("{}", "❌ Connection string cannot be empty.".red());
                anyhow::bail!("MongoDB connection string is required");
            }

            config.mongodb_url = connection_string.trim().to_string();

            // Validate the connection
            println!();
            println!("{}", "🔍 Validating connection...".cyan());
            match MongoStore::new(&config).await {
                Ok(_) => {
                    println!("{}", "✅ Connection validated successfully".green());
                }
                Err(e) => {
                    println!("{}", format!("❌ Connection validation failed: {}", e).red());
                    println!("{}", "💡 Please check your connection string and try again.".yellow());
                    anyhow::bail!("MongoDB connection validation failed");
                }
            }
        } else {
            // No existing connection, set up local Docker instance
            println!();
            println!("{}", "🐳 Setting up local MongoDB with Docker...".cyan());
            match start_docker_mongodb() {
                Ok(_) => {
                    config.mongodb_url = "mongodb://localhost:27017/".to_string();
                    println!("{}", "✅ Local MongoDB configured".green());
                }
                Err(e) => {
                    println!("{}", format!("❌ Docker setup failed: {}", e).red());
                    println!("{}", "💡 Please install Docker or choose another option.".yellow());
                    anyhow::bail!("Docker MongoDB setup failed");
                }
            }
        }
    } else {
        println!("{}", format!("│ ✅ MongoDB URL found: {} │", mask_connection_string(&config.mongodb_url)).bright_magenta());
        println!("{}", "└─────────────────────────────────────────────────────────┘".bright_magenta());

        if Confirm::new()
            .with_prompt("🔄 Would you like to change the MongoDB configuration?")
            .default(false)
            .interact()? {
            println!();
            let new_url: String = Input::new()
                .with_prompt("🔗 Enter new MongoDB connection URL")
                .default(config.mongodb_url.clone())
                .interact_text()?;

            if new_url.trim().is_empty() {
                println!("{}", "❌ Connection URL cannot be empty.".red());
                anyhow::bail!("MongoDB URL is required");
            }

            config.mongodb_url = new_url.trim().to_string();
            println!("{}", "✅ MongoDB configuration updated".green());
        }
    }

    // Step 4: Test MongoDB Connection
    println!();
    println!("{}", "┌─ Step 4: Testing Database Connection ────────────────────┐".bright_green());
    println!("{}", "│                                                         │".bright_green());
    println!("{}", format!("│ 🔍 Testing: {} │", mask_connection_string(&config.mongodb_url)).bright_green());
    println!("{}", "│                                                         │".bright_green());
    println!("{}", "└─────────────────────────────────────────────────────────┘".bright_green());

    match MongoStore::new(&config).await {
        Ok(_) => {
            println!("{}", "✅ Database connection successful".green().bold());
        }
        Err(e) => {
            println!("{}", format!("❌ Database connection failed: {}", e).red());
            println!("{}", "💡 Please check your MongoDB configuration and try again.".yellow());
            anyhow::bail!("Database connection failed");
        }
    }

    // Step 5: Save Configuration
    println!();
    println!("{}", "┌─ Step 5: Saving Configuration ──────────────────────────┐".bright_white());
    println!("{}", "│                                                         │".bright_white());

    match config.save() {
        Ok(_) => {
            println!("{}", "│ 💾 Configuration saved to .env file                    │".bright_white());
            println!("{}", "│                                                         │".bright_white());
            println!("{}", "└─────────────────────────────────────────────────────────┘".bright_white());
        }
        Err(e) => {
            println!("{}", format!("❌ Failed to save configuration: {}", e).red());
            anyhow::bail!("Configuration save failed");
        }
    }

    // Success message
    println!();
    println!("{}", "╔═══════════════════════════════════════╗".bright_green());
    println!("{}", "║           🎉 Setup Complete!          ║".bright_green().bold());
    println!("{}", "║                                       ║".bright_green());
    println!("{}", "║ You're ready to analyze repositories! ║".bright_green());
    println!("{}", "╚═══════════════════════════════════════╝".bright_green());
    println!();
    println!("{}", "💡 Try: whisper add <repo-url> or whisper <repo-url>".bright_cyan());
    println!();

    Ok(())
}

fn start_docker_mongodb() -> Result<()> {
    use std::process::Command;

    println!("{}", "🐳 Checking Docker availability...".cyan());

    // Check if Docker is available
    let docker_check = Command::new("docker")
        .arg("--version")
        .output();

    if docker_check.is_err() {
        println!("{}", "❌ Docker not found on your system.".red());
        println!("{}", "💡 Please install Docker from https://docker.com".yellow());
        println!("{}", "   Or choose MongoDB Atlas option (2) for cloud database.".yellow());
        anyhow::bail!("Docker not found. Please install Docker or provide a different MongoDB URL.");
    }

    println!("{}", "✅ Docker found".green());

    // Check if Docker daemon is running
    let daemon_check = Command::new("docker")
        .args(&["info"])
        .output();

    if daemon_check.is_err() || !daemon_check.as_ref().unwrap().status.success() {
        println!("{}", "❌ Docker daemon is not running.".red());
        println!("{}", "💡 Please start Docker Desktop or Docker daemon.".yellow());
        println!("{}", "   On macOS: Open Docker Desktop application".yellow());
        println!("{}", "   On Linux: Run 'sudo systemctl start docker'".yellow());
        anyhow::bail!("Docker daemon not running");
    }

    println!("{}", "✅ Docker daemon is running".green());

    // Check if port 27017 is already in use
    println!("{}", "🔍 Checking if port 27017 is available...".cyan());
    let port_check = Command::new("lsof")
        .args(&["-i", ":27017"])
        .output();

    if port_check.is_ok() && !port_check.as_ref().unwrap().stdout.is_empty() {
        println!("{}", "⚠️  Port 27017 is already in use.".yellow());
        println!("{}", "   This might be another MongoDB instance or the container is already running.".yellow());
        println!("{}", "   You can either:".yellow());
        println!("{}", "   1. Stop the existing service using port 27017".yellow());
        println!("{}", "   2. Use a different port for this container".yellow());
        println!("{}", "   3. Use an existing MongoDB connection instead".yellow());
        println!();
    } else {
        println!("{}", "✅ Port 27017 is available".green());
    }

    // Check if container exists
    println!("{}", "🔍 Checking for existing MongoDB container...".cyan());
    let container_check = Command::new("docker")
        .args(&["ps", "-a", "--filter", "name=git-whisperer-mongo", "--format", "{{.Names}}"])
        .output()?;

    let container_exists = String::from_utf8_lossy(&container_check.stdout).contains("git-whisperer-mongo");

    if container_exists {
        // Check if container is running
        let status_check = Command::new("docker")
            .args(&["ps", "--filter", "name=git-whisperer-mongo", "--format", "{{.Status}}"])
            .output()?;

        let is_running = !String::from_utf8_lossy(&status_check.stdout).is_empty();

        if is_running {
            println!("{}", "✅ MongoDB container is already running".green());
        } else {
            println!("{}", "📦 MongoDB container exists but is stopped, starting it...".yellow());
            let start_result = Command::new("docker")
                .args(&["start", "git-whisperer-mongo"])
                .output()?;

            if !start_result.status.success() {
                let stderr = String::from_utf8_lossy(&start_result.stderr);
                println!("{}", "❌ Failed to start existing container.".red());
                println!("{}", format!("   Docker error: {}", stderr.trim()).red());
                println!("{}", "💡 Trying to remove and recreate the container...".yellow());

                // Remove the failed container
                let _ = Command::new("docker")
                    .args(&["rm", "-f", "git-whisperer-mongo"])
                    .output();

                // Now try to create a new one
                println!("{}", "📦 Creating fresh MongoDB container...".yellow());
                let run_result = Command::new("docker")
                    .args(&[
                        "run", "-d",
                        "--name", "git-whisperer-mongo",
                        "-p", "27017:27017",
                        "mongo:7.0"
                    ])
                    .output()?;

                if !run_result.status.success() {
                    let stderr = String::from_utf8_lossy(&run_result.stderr);
                    println!("{}", "❌ Failed to create MongoDB container.".red());
                    println!("{}", format!("   Docker error: {}", stderr.trim()).red());
                    println!();
                    println!("{}", "💡 Troubleshooting suggestions:".yellow());
                    println!("{}", "   1. Make sure Docker Desktop is running".yellow());
                    println!("{}", "   2. Try: docker system prune -a (removes unused containers)".yellow());
                    println!("{}", "   3. Check if another MongoDB is using port 27017".yellow());
                    println!("{}", "   4. Use MongoDB Atlas instead (cloud, free tier)".yellow());
                    println!();
                    anyhow::bail!("Failed to create MongoDB container");
                }
            }
        }
    } else {
        println!("{}", "📦 Creating and starting MongoDB container...".yellow());
        let run_result = Command::new("docker")
            .args(&[
                "run", "-d",
                "--name", "git-whisperer-mongo",
                "-p", "27017:27017",
                "mongo:7.0"
            ])
            .output()?;

        if !run_result.status.success() {
            let stderr = String::from_utf8_lossy(&run_result.stderr);
            println!("{}", "❌ Failed to create MongoDB container.".red());
            println!("{}", format!("   Docker error: {}", stderr.trim()).red());
            println!();
            println!("{}", "💡 Troubleshooting suggestions:".yellow());
            println!("{}", "   1. Make sure Docker Desktop is running".yellow());
            println!("{}", "   2. Try: docker system prune -a (removes unused containers)".yellow());
            println!("{}", "   3. Check if another MongoDB is using port 27017".yellow());
            println!("{}", "   4. Use MongoDB Atlas instead (cloud, free tier)".yellow());
            println!();
            anyhow::bail!("Failed to create MongoDB container");
        }
    }

    println!("{}", "✅ Local MongoDB started successfully".green());
    println!("{}", "⏳ Waiting for MongoDB to be ready...".yellow());

    // Progress indicator for waiting
    for i in 1..=3 {
        print!("{}", format!("   {} second{}...", i, if i == 1 { "" } else { "s" }).cyan());
        std::io::stdout().flush()?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        print!("\r");
    }
    println!("{}", "   ✅ MongoDB is ready!".green());

    Ok(())
}

fn mask_connection_string(url: &str) -> String {
    // Mask password in connection string for display
    if url.contains("@") {
        if let Some(at_pos) = url.find('@') {
            if let Some(protocol_end) = url.find("://") {
                let protocol = &url[..protocol_end + 3];
                let host = &url[at_pos..];
                return format!("{}***:***{}", protocol, host);
            }
        }
    }
    url.to_string()
}
