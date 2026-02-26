use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "git-rewang")]
#[command(author = "Git Rewang Team")]
#[command(version = "0.1.0")]
#[command(about = "A lightweight CLI tool for developers who use multiple GitHub accounts", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a commit with template support
    Commit {
        /// Commit message
        message: String,
        
        /// Commit type (feat, fix, docs, style, refactor, test, chore)
        #[arg(short = 't', long = "type", default_value = "feat")]
        commit_type: String,
    },
    
    /// Show repository status in a simplified format
    Status,
    
    /// Clean merged local branches
    #[command(name = "clean-branches")]
    CleanBranches,
    
    /// List all local branches
    #[command(name = "list-branches")]
    ListBranches,
}

pub fn parse() -> Cli {
    Cli::parse()
}
