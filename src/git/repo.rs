use git2::Repository;
use anyhow::{Result, anyhow};

/// Open the current git repository
pub fn open_repo() -> Result<Repository> {
    Repository::discover(".")
        .map_err(|_| anyhow!("Not inside a git repository"))
}

/// Get the current branch name
pub fn get_current_branch(repo: &Repository) -> Result<String> {
    let head = repo.head()
        .map_err(|_| anyhow!("Failed to get HEAD reference"))?;
    
    Ok(head.shorthand().unwrap_or("unknown").to_string())
}

/// Get the default branch name (main or master)
pub fn get_default_branch(repo: &Repository) -> Result<String> {
    // Check for main first
    if repo.find_branch("main", git2::BranchType::Local).is_ok() {
        return Ok("main".to_string());
    }
    
    // Fall back to master
    if repo.find_branch("master", git2::BranchType::Local).is_ok() {
        return Ok("master".to_string());
    }
    
    Err(anyhow!("Could not find default branch (main or master)"))
}

/// Check if there are staged files
pub fn has_staged_files(repo: &Repository) -> Result<bool> {
    let index = repo.index()
        .map_err(|e| anyhow!("Failed to get index: {}", e))?;
    
    Ok(index.len() > 0)
}

/// Get repository info for display
pub struct RepoInfo {
    pub current_branch: String,
    pub has_staged: bool,
    pub is_clean: bool,
}

pub fn get_repo_info(repo: &Repository) -> Result<RepoInfo> {
    let current_branch = get_current_branch(repo)?;
    let has_staged = has_staged_files(repo)?;
    
    let statuses = repo.statuses(None)
        .map_err(|e| anyhow!("Failed to get statuses: {}", e))?;
    
    let is_clean = statuses.is_empty();
    
    Ok(RepoInfo {
        current_branch,
        has_staged,
        is_clean,
    })
}
