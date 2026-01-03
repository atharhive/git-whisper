use anyhow::{Result, Context};
use colored::*;
use crate::workspace::Workspace;
use crate::repository::GitAnalyzer;
use crate::gemini::GeminiClient;
use crate::config::Config;

pub async fn run_last(count: usize, repo: Option<&str>) -> Result<()> {
    let path = get_repo_path(repo)?;
    
    println!("\n{}", format!("🔍 Analyzing last {} commits...", count).blue().bold());
    
    let analyzer = GitAnalyzer::new(&path)?;
    let mut commits = analyzer.get_commits()?;
    commits.truncate(count);
    
    let config = Config::load()?;
    let client = GeminiClient::new(&config.gemini_api_key)?;
    
    let prompt = format!(
        "Explain the most recent work in this project and how it fits into the bigger picture. \
         Focus on intent and impact, not implementation details.\n\n{}",
        format_commits(&commits)
    );
    
    let explanation = client.generate_content(&prompt).await?;
    
    println!("\n{}", "═".repeat(60).magenta());
    println!("{}", format!("  LAST {} COMMITS", count).magenta().bold());
    println!("{}", "═".repeat(60).magenta());
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
