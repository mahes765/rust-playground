mod git;

use std::env;
use git::commit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: git-helper <type> <message>");
        return;
    }

    let commit_message = &args[2];
    let commit_type = &args[1];

    if let Err(e) = commit::run_commit(commit_message, commit_type) {
        eprintln!("Error: {}", e);
    }
}