use colored::*;

pub fn show_welcome() {
    println!("\n{}", "╔═══════════════════════════════════════╗".bright_blue());
    println!("{}", "║     🎭 Whisper                       ║".bright_blue());
    println!("{}", "╚═══════════════════════════════════════╝".bright_blue());
    
    println!("\n{}", "Turn commit history into human stories".cyan());
    
    println!("\n{}", "Commands:".green().bold());
    println!("  whisper add <repo-url>        Add a repository for analysis");
    println!("  whisper summary               Full project story from git history");
    println!("  whisper demo                  60-90 second demo script");
    println!("  whisper last [count]          Explain recent work (default: 5 commits)");
    println!("  whisper since <ref>           Changes since commit/tag/date");
    println!("  whisper changelog             Clean changelog grouped by type");
    println!("  whisper setup                 Configure API keys and database");
    
    println!("\n{}", "Examples:".green().bold());
    println!("  whisper add https://github.com/user/repo");
    println!("  whisper summary");
    println!("  whisper demo");
    println!("  whisper last 10");
    println!("  whisper since v1.0.0");
    println!("  whisper since 2024-01-01");
    
    println!("\n{}", "Quick mode:".yellow().bold());
    println!("  whisper <repo-url>            Add repo and show summary");
    
    println!("\n{}", "Get your API key: https://makersuite.google.com/app/apikey".dimmed());
    println!();
}
