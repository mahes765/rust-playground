use git2::BranchType;
use anyhow::{Result, anyhow};
use colored::Colorize;

use crate::config::load_config;
use super::repo::{open_repo, get_current_branch};

/// Clean merged branches
pub fn clean_branches() -> Result<()> {
    let repo = open_repo()?;
    let config = load_config()?;
    let default_branch = &config.default_branch;
    let current_branch = get_current_branch(&repo)?;
    
    // Get default branch commit
    let default_branch_ref = repo.find_branch(default_branch, BranchType::Local)
        .map_err(|_| anyhow!("Default branch '{}' not found", default_branch))?;
    
    let default_commit = default_branch_ref
        .get()
        .peel_to_commit()
        .map_err(|_| anyhow!("Failed to get default branch commit"))?;
    
    let branches = repo.branches(Some(BranchType::Local))
        .map_err(|e| anyhow!("Failed to list branches: {}", e))?;
    
    let mut deleted_count = 0;
    let mut skipped: Vec<String> = Vec::new();
    
    for branch_result in branches {
        let (branch, _) = branch_result
            .map_err(|e| anyhow!("Failed to iterate branches: {}", e))?;
        
        let branch_name = branch.name()
            .map_err(|_| anyhow!("Failed to get branch name"))?
            .unwrap_or("unknown")
            .to_string();
        
        // Skip current branch and default branch
        if branch_name == current_branch || branch_name == *default_branch {
            skipped.push(branch_name);
            continue;
        }
        
        // Check if branch is merged into default
        let branch_commit = branch.get()
            .peel_to_commit()
            .map_err(|_| anyhow!("Failed to get branch commit"))?;
        
        let is_merged = repo.merge_base(default_commit.id(), branch_commit.id())
            .map(|merge_base| merge_base == branch_commit.id())
            .unwrap_or(false);
        
        if is_merged {
            // Delete the branch
            let mut branch = branch;
            branch.delete()
                .map_err(|e| anyhow!("Failed to delete branch '{}': {}", branch_name, e))?;
            
            println!("{} {}", "Deleted:".green(), branch_name);
            deleted_count += 1;
        } else {
            skipped.push(branch_name);
        }
    }
    
    println!("\n{}", "Summary:".bold());
    println!("  Deleted: {} branches", deleted_count);
    println!("  Skipped: {} branches", skipped.len());
    
    if !skipped.is_empty() {
        println!("\n{}", "Skipped branches:".yellow());
        for name in skipped {
            println!("  - {}", name);
        }
    }
    
    Ok(())
}

/// List all local branches
pub fn list_branches() -> Result<()> {
    let repo = open_repo()?;
    let current_branch = get_current_branch(&repo)?;
    
    let branches = repo.branches(Some(BranchType::Local))
        .map_err(|e| anyhow!("Failed to list branches: {}", e))?;
    
    println!("{}", "Local branches:".bold());
    
    for branch_result in branches {
        let (branch, _) = branch_result
            .map_err(|e| anyhow!("Failed to iterate branches: {}", e))?;
        
        let branch_name = branch.name()
            .map_err(|_| anyhow!("Failed to get branch name"))?
            .unwrap_or("unknown");
        
        if branch_name == current_branch {
            println!("  {} {}", "*".green(), branch_name.green());
        } else {
            println!("    {}", branch_name);
        }
    }
    
    Ok(())
}
