use anyhow::{Result, Context};
use colored::*;
use crate::workspace::Workspace;
use crate::repository::GitAnalyzer;
use crate::gemini::GeminiClient;
use crate::config::Config;

pub async fn run_since(reference: &str, repo: Option<&str>) -> Result<()> {
    let path = get_repo_path(repo)?;
    
    println!("\n{}", format!("📅 Changes since '{}'...", reference).blue().bold());
    
    let analyzer = GitAnalyzer::new(&path)?;
    let all_commits = analyzer.get_commits()?;
    
    // Find commits since reference
    let since_commits: Vec<_> = all_commits.iter()
        .take_while(|c| !c.hash.starts_with(reference) && !c.message.contains(reference))
        .cloned()
        .collect();
    
    if since_commits.is_empty() {
        println!("{}", "No changes found since that reference.".yellow());
        return Ok(());
    }
    
    let config = Config::load()?;
    let client = GeminiClient::new(&config.gemini_api_key)?;
    
    let prompt = format!(
        "Explain what changed since '{}' in plain English. Focus on features, fixes, and impact.\n\n{}",
        reference,
        format_commits(&since_commits)
    );
    
    let explanation = client.generate_content(&prompt).await?;
    
    println!("\n{}", "═".repeat(60).green());
    println!("{}", format!("  CHANGES SINCE '{}'", reference).green().bold());
    println!("{}", "═".repeat(60).green());
    println!("\n{}\n", explanation);
    
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

fn format_commits(commits: &[crate::repository::Commit]) -> String {
    commits.iter()
        .map(|c| format!("- {}: {}", &c.hash[..7], c.message))
        .collect::<Vec<_>>()
        .join("\n")
}
