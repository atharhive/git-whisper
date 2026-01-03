use clap::{Parser, Subcommand};
use anyhow::Result;

mod cli;
mod repository;
mod config;
mod gemini;
mod storage;
mod workspace;

#[derive(Parser)]
#[command(name = "whisper")]
#[command(about = "Turn commit history into human stories", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Path to git repository or URL
    #[arg(value_name = "REPO_PATH")]
    repo_path: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run interactive setup wizard
    Setup,
    
    /// Add a repository for analysis (clones if URL)
    Add {
        /// Repository URL or path
        repo: String,
    },
    
    /// Generate full project summary from entire git history
    Summary {
        /// Repository path (uses last added if omitted)
        #[arg(short, long)]
        repo: Option<String>,
    },
    
    /// Generate a 60-90 second demo script from recent commits
    Demo {
        /// Repository path (uses last added if omitted)
        #[arg(short, long)]
        repo: Option<String>,
    },
    
    /// Explain what changed since a commit, tag, or date
    Since {
        /// Commit hash, tag, or date (e.g., "v1.0.0", "2024-01-01")
        reference: String,
        
        /// Repository path (uses last added if omitted)
        #[arg(short, long)]
        repo: Option<String>,
    },
    
    /// Explain the most recent work and how it fits the bigger picture
    Last {
        /// Number of commits to analyze (default: 5)
        #[arg(short, long, default_value = "5")]
        count: usize,
        
        /// Repository path (uses last added if omitted)
        #[arg(short, long)]
        repo: Option<String>,
    },
    
    /// Generate a clean changelog grouped by features, fixes, and refactors
    Changelog {
        /// Repository path (uses last added if omitted)
        #[arg(short, long)]
        repo: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Setup) => {
            cli::setup::run_setup().await?;
        }
        Some(Commands::Add { repo }) => {
            cli::add::run_add(&repo).await?;
        }
        Some(Commands::Summary { repo }) => {
            cli::summary::run_summary(repo.as_deref()).await?;
        }
        Some(Commands::Demo { repo }) => {
            cli::demo::run_demo(repo.as_deref()).await?;
        }
        Some(Commands::Since { reference, repo }) => {
            cli::since::run_since(&reference, repo.as_deref()).await?;
        }
        Some(Commands::Last { count, repo }) => {
            cli::last::run_last(count, repo.as_deref()).await?;
        }
        Some(Commands::Changelog { repo }) => {
            cli::changelog::run_changelog(repo.as_deref()).await?;
        }
        None => {
            if let Some(path) = cli.repo_path {
                // Quick analysis mode
                cli::add::run_add(&path).await?;
                cli::summary::run_summary(None).await?;
            } else {
                cli::help::show_welcome();
            }
        }
    }
    
    Ok(())
}
