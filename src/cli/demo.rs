use anyhow::{Result, Context};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use crate::workspace::Workspace;
use crate::repository::GitAnalyzer;
use crate::gemini::GeminiClient;
use crate::config::Config;

pub async fn run_demo(repo: Option<&str>) -> Result<()> {
    let path = get_repo_path(repo)?;
    
    println!("\n{}", "🎬 Generating demo script...".blue().bold());
    
    let analyzer = GitAnalyzer::new(&path)?;
    let mut commits = analyzer.get_commits()?;
    
    // Take last 20 commits for demo
    commits.truncate(20);
    
    let config = Config::load_or_setup().await?;
    let client = GeminiClient::new(&config.gemini_api_key)?;
    
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}").unwrap());
    spinner.set_message("🤖 Crafting your demo script...");
    
    let prompt = format!(
        "Generate a 60-90 second demo script from these recent commits. \
         Focus on the story: what problem was solved, key decisions, and impact. \
         Make it conversational and demo-ready.\n\n{}",
        format_commits(&commits)
    );
    
    let demo = client.generate_content(&prompt).await?;
    spinner.finish_with_message("✅ Done".green().to_string());
    
    println!("\n{}", "═".repeat(60).cyan());
    println!("{}", "  DEMO SCRIPT (60-90 seconds)".cyan().bold());
    println!("{}", "═".repeat(60).cyan());
    println!("\n{}\n", demo);
    
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
