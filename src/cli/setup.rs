use anyhow::Result;
use colored::*;
use dialoguer::{Input, Confirm, Password};

use crate::config::Config;
use crate::gemini::GeminiClient;
use crate::storage::MongoStore;

pub async fn run_setup() -> Result<()> {
    println!("\n{}", "🔧 Git Whisperer Setup".blue().bold());
    println!();
    
    let mut config = Config::load_or_default()?;
    
    // Gemini API Key
    if config.gemini_api_key.is_empty() {
        println!("{}", "🤖 Gemini API Key Required".yellow());
        println!("Get your API key from: https://makersuite.google.com/app/apikey\n");
        
        let api_key: String = Password::new()
            .with_prompt("Enter your Gemini API key")
            .interact()?;
        
        if api_key.is_empty() {
            anyhow::bail!("Gemini API key is required");
        }
        
        config.gemini_api_key = api_key;
    } else {
        println!("{}", "✅ Gemini API key found".green());
    }
    
    // Test Gemini API
    println!("\n{}", "🔍 Testing Gemini API connection...".yellow());
    let client = GeminiClient::new(&config.gemini_api_key)?;
    client.test_connection().await?;
    println!("{}", "✅ Gemini API connection successful".green());
    
    // MongoDB Configuration
    println!("\n{}", "🗄️  MongoDB Configuration".yellow());
    
    if config.mongodb_url.is_empty() {
        println!("\nChoose your MongoDB setup:");
        println!("  1. Local MongoDB with Docker (easiest)");
        println!("  2. MongoDB Atlas (cloud, free tier available)");
        println!("  3. Custom MongoDB URL\n");
        
        let choice: String = Input::new()
            .with_prompt("Enter your choice (1-3)")
            .default("1".to_string())
            .interact_text()?;
        
        match choice.as_str() {
            "1" => {
                // Local Docker MongoDB
                start_docker_mongodb()?;
                config.mongodb_url = "mongodb://localhost:27017/".to_string();
            }
            "2" => {
                // MongoDB Atlas
                println!("\n{}", "📚 MongoDB Atlas Setup:".cyan());
                println!("  1. Go to https://www.mongodb.com/cloud/atlas/register");
                println!("  2. Create a free cluster");
                println!("  3. Create a database user");
                println!("  4. Get your connection string\n");
                
                let atlas_url: String = Input::new()
                    .with_prompt("Enter your MongoDB Atlas connection string")
                    .interact_text()?;
                
                config.mongodb_url = atlas_url;
            }
            "3" => {
                // Custom URL
                let custom_url: String = Input::new()
                    .with_prompt("Enter MongoDB connection URL")
                    .default("mongodb://localhost:27017/".to_string())
                    .interact_text()?;
                
                config.mongodb_url = custom_url;
            }
            _ => {
                anyhow::bail!("Invalid choice. Please run setup again.");
            }
        }
    } else {
        println!("{}", format!("✅ MongoDB URL found: {}", mask_connection_string(&config.mongodb_url)).green());
        
        if Confirm::new()
            .with_prompt("Would you like to change it?")
            .default(false)
            .interact()? 
        {
            let new_url: String = Input::new()
                .with_prompt("Enter new MongoDB connection URL")
                .default(config.mongodb_url.clone())
                .interact_text()?;
            
            config.mongodb_url = new_url;
        }
    }
    
    // Test MongoDB connection
    println!("\n{}", format!("🔍 Testing MongoDB connection: {}", mask_connection_string(&config.mongodb_url)).yellow());
    let _store = MongoStore::new(&config).await?;
    println!("{}", "✅ MongoDB connection successful".green());
    
    // Save configuration
    config.save()?;
    println!("\n{}", "💾 Configuration saved to .env file".green());
    println!("\n{}", "🎉 Setup complete! You're ready to analyze repositories.".green().bold());
    println!();
    
    Ok(())
}

fn start_docker_mongodb() -> Result<()> {
    use std::process::Command;
    
    println!("{}", "🐳 Setting up local MongoDB with Docker...".yellow());
    
    // Check if Docker is available
    let docker_check = Command::new("docker")
        .arg("--version")
        .output();
    
    if docker_check.is_err() {
        anyhow::bail!("Docker not found. Please install Docker or provide a different MongoDB URL.");
    }
    
    println!("{}", "✅ Docker found".green());
    
    // Check if container exists
    let container_check = Command::new("docker")
        .args(&["ps", "-a", "--filter", "name=git-whisperer-mongo", "--format", "{{.Names}}"])
        .output()?;
    
    let container_exists = String::from_utf8_lossy(&container_check.stdout).contains("git-whisperer-mongo");
    
    if container_exists {
        println!("{}", "📦 MongoDB container exists, starting it...".yellow());
        Command::new("docker")
            .args(&["start", "git-whisperer-mongo"])
            .output()?;
    } else {
        println!("{}", "📦 Creating and starting MongoDB container...".yellow());
        Command::new("docker")
            .args(&[
                "run", "-d",
                "--name", "git-whisperer-mongo",
                "-p", "27017:27017",
                "mongo:7.0"
            ])
            .output()?;
    }
    
    println!("{}", "✅ Local MongoDB started successfully".green());
    println!("{}", "⏳ Waiting for MongoDB to be ready...".yellow());
    std::thread::sleep(std::time::Duration::from_secs(3));
    
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
