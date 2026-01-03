use anyhow::{Result, Context};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

use crate::config::Config;
use crate::repository::GitAnalyzer;
use crate::storage::MongoStore;
use crate::gemini::GeminiClient;

pub async fn run_analysis(repo_path: &str, generate_changelog: bool) -> Result<()> {
    // Load configuration
    let config = Config::load_or_setup().await?;
    
    // Check if it's a URL or local path
    let is_url = repo_path.starts_with("http://") || repo_path.starts_with("https://") || repo_path.starts_with("git@");
    
    let temp_dir;
    let actual_path = if is_url {
        println!("\n{}", "🌐 Detected repository URL".blue().bold());
        println!("{}", format!("📥 Cloning: {}", repo_path).cyan());
        
        // Clone to temporary directory
        temp_dir = tempfile::tempdir()?;
        let clone_path = temp_dir.path();
        
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
        );
        spinner.set_message("Cloning repository...");
        
        clone_repository(repo_path, clone_path)?;
        spinner.finish_with_message("✅ Repository cloned".green().to_string());
        
        clone_path.to_str().unwrap()
    } else {
        repo_path
    };
    
    println!("\n{}", "🔍 Starting analysis of repository".blue().bold());
    println!("{}", format!("📁 Path: {}", actual_path).cyan());
    println!();
    
    // Analyze git repository
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    
    spinner.set_message("📚 Fetching git history...");
    let analyzer = GitAnalyzer::new(actual_path)?;
    let commits = analyzer.get_commits()?;
    spinner.finish_with_message(format!("✅ Found {} commits", commits.len()).green().to_string());
    
    // Store in database
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("💾 Storing analysis in database...");
    let store = MongoStore::new(&config).await?;
    store.save_commits(&commits).await?;
    spinner.finish_with_message("✅ Successfully stored commits".green().to_string());
    
    // Generate story with AI
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("🤖 Generating project story with AI...");
    let client = GeminiClient::new(&config.gemini_api_key)?;
    let story = client.generate_project_summary(&commits).await?;
    spinner.finish_with_message("✅ Successfully generated project story!".green().to_string());
    
    // Generate changelog if requested
    let changelog = if generate_changelog {
        let spinner = ProgressBar::new_spinner();
        spinner.set_message("📝 Generating changelog...");
        let changelog = client.generate_changelog(&commits).await?;
        spinner.finish_with_message("✅ Successfully generated changelog!".green().to_string());
        Some(changelog)
    } else {
        None
    };
    
    // Print results
    print_results(repo_path, &commits, &story, changelog.as_deref(), &config);
    
    Ok(())
}

fn clone_repository(url: &str, path: &std::path::Path) -> Result<()> {
    use git2::Repository;
    
    Repository::clone(url, path)
        .context("Failed to clone repository. Check the URL and your network connection.")?;
    
    Ok(())
}

fn print_results(repo_path: &str, commits: &[crate::repository::Commit], story: &str, changelog: Option<&str>, config: &Config) {
    println!("\n{}", "═".repeat(60).magenta());
    println!("{}", "  GIT WHISPERER ANALYSIS RESULTS".magenta().bold());
    println!("{}", "═".repeat(60).magenta());
    
    println!("\n{}", "┌─ Analysis Summary ─────────────────────────────┐".green());
    println!("{}", format!("│ 📁 Repository: {:<35} │", repo_path).green());
    println!("{}", format!("│ 📊 Commits analyzed: {:<28} │", commits.len()).green());
    println!("{}", format!("│ 🤖 AI Model: {:<35} │", "Gemini 2.5 Flash").green());
    println!("{}", format!("│ 💾 Database: {:<35} │", config.mongodb_db).green());
    println!("{}", "└────────────────────────────────────────────────┘".green());
    
    println!("\n{}", "┌─ 📖 Project Story ─────────────────────────────┐".yellow());
    for line in story.lines() {
        println!("{}", format!("│ {:<47} │", line).yellow());
    }
    println!("{}", "└────────────────────────────────────────────────┘".yellow());
    
    if let Some(changelog_text) = changelog {
        println!("\n{}", "┌─ 📝 Changelog ─────────────────────────────────┐".cyan());
        for line in changelog_text.lines() {
            println!("{}", format!("│ {:<47} │", line).cyan());
        }
        println!("{}", "└────────────────────────────────────────────────┘".cyan());
    }
    
    println!("\n{}", "✨ Analysis complete! Your project story is ready.".green().bold());
    println!();
}
