use anyhow::{Result, Context};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use crate::workspace::Workspace;
use crate::repository::GitAnalyzer;
use crate::gemini::GeminiClient;
use crate::config::Config;

pub async fn run_summary(repo: Option<&str>) -> Result<()> {
    let path = get_repo_path(repo)?;
    
    println!("\n{}", "📖 Generating project summary...".blue().bold());
    
    let analyzer = GitAnalyzer::new(&path)?;
    let commits = analyzer.get_commits()?;
    
    let config = Config::load_or_setup().await?;
    let client = GeminiClient::new(&config.gemini_api_key)?;
    
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}").unwrap());
    spinner.set_message("🤖 AI is analyzing your project...");
    
    let summary = client.generate_project_summary(&commits).await?;
    spinner.finish_with_message("✅ Done".green().to_string());
    
    println!("\n{}", "═".repeat(60).yellow());
    println!("{}", "  PROJECT SUMMARY".yellow().bold());
    println!("{}", "═".repeat(60).yellow());
    println!("\n{}\n", summary);
    
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
