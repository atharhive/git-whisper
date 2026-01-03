use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use crate::workspace::Workspace;
use crate::repository::GitAnalyzer;
use crate::storage::MongoStore;
use crate::config::Config;

pub async fn run_add(repo: &str) -> Result<()> {
    let is_url = repo.starts_with("http://") || repo.starts_with("https://") || repo.starts_with("git@");
    
    let (actual_path, repo_name, url) = if is_url {
        println!("\n{}", "🌐 Cloning repository...".blue().bold());
        
        let temp_dir = tempfile::tempdir()?;
        let clone_path = temp_dir.path();
        
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}").unwrap());
        spinner.set_message("Cloning...");
        
        git2::Repository::clone(repo, clone_path)?;
        spinner.finish_with_message("✅ Cloned".green().to_string());
        
        let name = repo.split('/').last().unwrap_or("repo").trim_end_matches(".git");
        (clone_path.to_str().unwrap().to_string(), name.to_string(), Some(repo.to_string()))
    } else {
        let name = std::path::Path::new(repo).file_name().unwrap().to_str().unwrap();
        (repo.to_string(), name.to_string(), None)
    };
    
    println!("{}", format!("📁 Analyzing: {}", repo_name).cyan());
    
    // Analyze
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("📚 Reading git history...");
    let analyzer = GitAnalyzer::new(&actual_path)?;
    let commits = analyzer.get_commits()?;
    spinner.finish_with_message(format!("✅ Found {} commits", commits.len()).green().to_string());
    
    // Store
    let config = Config::load_or_setup().await?;
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("💾 Storing...");
    let store = MongoStore::new(&config).await?;
    store.save_commits(&commits).await?;
    spinner.finish_with_message("✅ Stored".green().to_string());
    
    // Save to manager
    let manager = Workspace::new()?;
    manager.add_repo(repo_name.clone(), actual_path, url)?;
    
    println!("\n{}", format!("✨ Repository '{}' added successfully!", repo_name).green().bold());
    println!("{}", "Now you can use: whisper summary, whisper demo, etc.".dimmed());
    
    Ok(())
}
