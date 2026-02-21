use git2::{Repository, Signature};
use chrono::Local;
use anyhow::{Result, anyhow};

pub fn run_commit(massage_type: &str, commit_info: &str) ->Result<()> {
    
    let repo = Repository::discover(".")
        .map_err(|e| anyhow!("Not inside a git repository"))?;

    let mut index = repo.index()?;
    
    if index.len() == 0 {
        return Err(anyhow!("No Staged files found"));
    }

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let head = repo.head()?;
    let brach_nama = head.shorthand().unwrap_or("Unknow");

    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    let commit_message = format!("{}: {}\n\nbrach: {}\ndate: {}", commit_type, massage_input, branch_name, now);

    let signature = repo.signature()
        .or_else(|_| Signature::now("git-rewang", "git-helper@example.com"))?;

    let parent_commit = match head.target() {
        Some(oid) => Some(repo.find_commit(oid)?),
        None => None,
    };

    match parent_commit {
        Some(parent) => {
            repo.commit(Some("HEAD"), &signature, &signature, &commit_message, &tree, &[&parent])?;
        },
        None => {
            repo.commit(Some("HEAD"), &signature, &signature, &commit_message, &tree, &[])?;
        }
    }

    println!("Commit created successfully: {}", commit_message);
    Ok(())
}