use git2::Status;
use anyhow::Result;
use colored::Colorize;

use super::repo::{open_repo, get_current_branch};

/// Status summary structure
pub struct StatusSummary {
    pub branch: String,
    pub staged: usize,
    pub modified: usize,
    pub untracked: usize,
    pub deleted: usize,
    pub renamed: usize,
    pub conflicted: usize,
}

/// Get repository status summary
pub fn get_status_summary() -> Result<StatusSummary> {
    let repo = open_repo()?;
    let branch = get_current_branch(&repo)?;
    
    let statuses = repo.statuses(None)?;
    
    let mut staged = 0;
    let mut modified = 0;
    let mut untracked = 0;
    let mut deleted = 0;
    let mut renamed = 0;
    let mut conflicted = 0;
    
    for entry in statuses.iter() {
        let status = entry.status();
        
        // Staged changes
        if status.intersects(Status::INDEX_NEW | Status::INDEX_MODIFIED | Status::INDEX_DELETED | Status::INDEX_RENAMED | Status::INDEX_TYPECHANGE) {
            staged += 1;
        }
        
        // Working directory modifications
        if status.contains(Status::WT_MODIFIED) {
            modified += 1;
        }
        
        // Untracked files
        if status.contains(Status::WT_NEW) {
            untracked += 1;
        }
        
        // Deleted files
        if status.contains(Status::WT_DELETED) {
            deleted += 1;
        }
        
        // Renamed files
        if status.contains(Status::WT_RENAMED) {
            renamed += 1;
        }
        
        // Conflicted files
        if status.contains(Status::CONFLICTED) {
            conflicted += 1;
        }
    }
    
    Ok(StatusSummary {
        branch,
        staged,
        modified,
        untracked,
        deleted,
        renamed,
        conflicted,
    })
}

/// Show repository status in a simplified format
pub fn show() -> Result<()> {
    let summary = get_status_summary()?;
    
    println!("{}", "Repository Status".bold());
    println!("{}", "─".repeat(30));
    println!("  branch:    {}", summary.branch.cyan());
    
    if summary.staged > 0 {
        println!("  staged:    {} {}", summary.staged.to_string().green(), "files".green());
    }
    
    if summary.modified > 0 {
        println!("  modified:  {} {}", summary.modified.to_string().yellow(), "files".yellow());
    }
    
    if summary.deleted > 0 {
        println!("  deleted:   {} {}", summary.deleted.to_string().red(), "files".red());
    }
    
    if summary.untracked > 0 {
        println!("  untracked: {} files", summary.untracked);
    }
    
    if summary.renamed > 0 {
        println!("  renamed:   {} files", summary.renamed);
    }
    
    if summary.conflicted > 0 {
        println!("  {} {} {}", "conflicted:".red().bold(), summary.conflicted.to_string().red().bold(), "files".red().bold());
    }
    
    // Check if clean
    let total = summary.staged + summary.modified + summary.untracked + summary.deleted + summary.renamed + summary.conflicted;
    if total == 0 {
        println!("\n  {}", "Working directory clean ✓".green());
    }
    
    Ok(())
}
