use std::env;
use std::path::PathBuf;

/// Find the project root by looking for .git directory
pub fn find_project_root() -> Option<PathBuf> {
    let current_dir = env::current_dir().ok()?;
    let mut path = current_dir.as_path();
    
    loop {
        let git_dir = path.join(".git");
        if git_dir.exists() {
            return Some(path.to_path_buf());
        }
        
        match path.parent() {
            Some(parent) => path = parent,
            None => return None,
        }
    }
}

/// Get the git-helper config directory for the current project
pub fn get_project_config_dir() -> Option<PathBuf> {
    find_project_root().map(|p| p.join(".git-helper"))
}

/// Get the global git-helper config directory
pub fn get_global_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("git-helper"))
}

/// Ensure a directory exists, creating it if necessary
pub fn ensure_dir_exists(path: &PathBuf) -> std::io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Check if we're inside a git repository
pub fn is_git_repo() -> bool {
    find_project_root().is_some()
}
