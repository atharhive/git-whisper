use anyhow::{Result, Context};
use colored::*;
use crate::workspace::Workspace;
use crate::repository::GitAnalyzer;
use crate::gemini::GeminiClient;
use crate::config::Config;

pub async fn run_changelog(repo: Option<&str>) -> Result<()> {
    let path = get_repo_path(repo)?;
    
    println!("\n{}", "📝 Generating changelog...".blue().bold());
    
    let analyzer = GitAnalyzer::new(&path)?;
    let commits = analyzer.get_commits()?;
    
    let config = Config::load_or_setup().await?;
    let client = GeminiClient::new(&config.gemini_api_key)?;
    
    let changelog = client.generate_changelog(&commits).await?;
    
    println!("\n{}", "═".repeat(60).cyan());
    println!("{}", "  CHANGELOG".cyan().bold());
    println!("{}", "═".repeat(60).cyan());
    println!("\n{}\n", changelog);
    
    Ok(())
}

fn get_repo_path(repo: Option<&str>) -> Result<String> {
    if let Some(path) = repo {
        return Ok(path.to_string());
    }
    
    let manager = Workspace::new()?;
    let last = manager.get_last_repo()?.context("No repository added. Use 'whisper add <repo>'")?;
    Ok(last.path)
}
