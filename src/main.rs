use clap::{Parser, Subcommand};
use anyhow::Result;

mod cli;
mod repository;
mod config;
mod gemini;
mod storage;

#[derive(Parser)]
#[command(name = "git-whisperer")]
#[command(about = "Turn commit history into human stories", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Path to git repository
    #[arg(value_name = "REPO_PATH")]
    repo_path: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run interactive setup wizard
    Setup,
    /// Analyze a git repository
    Analyze {
        /// Path to the git repository or URL
        path: String,
        
        /// Generate a CHANGELOG.md file
        #[arg(long)]
        changelog: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Setup) => {
            cli::setup::run_setup().await?;
        }
        Some(Commands::Analyze { path, changelog }) => {
            cli::analyze::run_analysis(&path, changelog).await?;
        }
        None => {
            if let Some(path) = cli.repo_path {
                cli::analyze::run_analysis(&path, false).await?;
            } else {
                cli::help::show_welcome();
            }
        }
    }
    
    Ok(())
}
