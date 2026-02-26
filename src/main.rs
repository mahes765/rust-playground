mod cli;
mod config;
mod git;
mod template;
mod utils;

use cli::Commands;

fn main() {
    let cli = cli::parse();
    
    let result = match cli.command {
        Commands::Commit { message, commit_type } => {
            git::commit::run_commit(&message, &commit_type)
        }
        Commands::Status => {
            git::status::show()
        }
        Commands::CleanBranches => {
            git::branch::clean_branches()
        }
        Commands::ListBranches => {
            git::branch::list_branches()
        }
    };
    
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
