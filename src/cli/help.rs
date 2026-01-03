use colored::*;

pub fn show_welcome() {
    println!("\n{}", "╔═══════════════════════════════════════╗".bright_blue());
    println!("{}", "  ║     🎭 Git Whisperer                 ║".bright_blue());
    println!("{}", "  ╚═══════════════════════════════════════╝".bright_blue());
    
    println!("\n{}", "Turn commit history into human stories".cyan());
    println!("\n{}", "Usage:".green().bold());
    println!("  git-whisperer <path-or-url>");
    println!("  git-whisperer analyze <path-or-url>");
    println!("  git-whisperer setup");
    
    println!("\n{}", "Examples:".green().bold());
    println!("  git-whisperer /path/to/your/project");
    println!("  git-whisperer .                                    {}", "# Current directory".dimmed());
    println!("  git-whisperer https://github.com/user/repo        {}", "# Clone and analyze".dimmed());
    println!("  git-whisperer git@github.com:user/repo.git        {}", "# SSH URL".dimmed());
    
    println!("\n{}", "Setup:".yellow().bold());
    println!("  • First run will guide you through configuration");
    println!("  • Gemini API key and MongoDB setup handled automatically");
    println!("  • Configuration saved for future runs");
    
    println!("\n{}", "Get your API key from: https://makersuite.google.com/app/apikey".dimmed());
    println!();
}
