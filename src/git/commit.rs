use git2::Signature;
use anyhow::{Result, anyhow};
use colored::Colorize;

use super::repo::{open_repo, get_current_branch};
use crate::template::loader::render_commit_message;

pub fn run_commit(message_input: &str, commit_type: &str) -> Result<()> {
    let repo = open_repo()?;
    
    let mut index = repo.index()?;
    
    // Check if there are staged changes
    let statuses = repo.statuses(None)?;
    let has_staged_changes = statuses.iter().any(|entry| {
        let status = entry.status();
        status.intersects(
            git2::Status::INDEX_NEW |
            git2::Status::INDEX_MODIFIED |
            git2::Status::INDEX_DELETED |
            git2::Status::INDEX_RENAMED |
            git2::Status::INDEX_TYPECHANGE
        )
    });
    
    if !has_staged_changes {
        return Err(anyhow!("Nothing to commit. No staged changes found.\nUse 'git add <file>' to stage changes first."));
    }

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let branch_name = get_current_branch(&repo)?;

    // Use template system for commit message
    let commit_message = render_commit_message(commit_type, message_input, &branch_name)?;

    let signature = repo.signature()
        .or_else(|_| Signature::now("git-rewang", "git-helper@example.com"))?;

    let head = repo.head()?;
    let parent_commit = match head.target() {
        Some(oid) => Some(repo.find_commit(oid)?),
        None => None,
    };
    
    // Check if the new tree is the same as parent tree (no actual changes)
    if let Some(ref parent) = parent_commit {
        if parent.tree_id() == tree_id {
            return Err(anyhow!("Nothing to commit. The staged files have no changes compared to the last commit."));
        }
    }

    match parent_commit {
        Some(parent) => {
            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                &commit_message,
                &tree,
                &[&parent],
            )?;
        }
        None => {
            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                &commit_message,
                &tree,
                &[],
            )?;
        }
    }

    println!("{}", "Commit created successfully!".green());
    println!("{}", "─".repeat(40));
    println!("{}", commit_message);
    
    Ok(())
}