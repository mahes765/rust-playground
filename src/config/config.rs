use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde::Deserialize;
use anyhow::{Result, anyhow};

use crate::utils::fs::find_project_root;

/// Main configuration structure
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_branch")]
    pub default_branch: String,
    
    #[serde(default)]
    pub commit_types: HashMap<String, String>,
    
    #[serde(default)]
    pub user: Option<UserConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserConfig {
    pub name: Option<String>,
    pub email: Option<String>,
}

fn default_branch() -> String {
    "main".to_string()
}

impl Default for Config {
    fn default() -> Self {
        let mut commit_types = HashMap::new();
        commit_types.insert("feat".to_string(), "feat: {message}".to_string());
        commit_types.insert("fix".to_string(), "fix: {message}".to_string());
        commit_types.insert("docs".to_string(), "docs: {message}".to_string());
        commit_types.insert("style".to_string(), "style: {message}".to_string());
        commit_types.insert("refactor".to_string(), "refactor: {message}".to_string());
        commit_types.insert("test".to_string(), "test: {message}".to_string());
        commit_types.insert("chore".to_string(), "chore: {message}".to_string());
        
        Config {
            default_branch: default_branch(),
            commit_types,
            user: None,
        }
    }
}

impl Config {
    /// Get the commit template for a given type
    pub fn get_commit_template(&self, commit_type: &str) -> Option<&String> {
        self.commit_types.get(commit_type)
    }
}

/// Load configuration from file
/// Priority: project config > global config > default
pub fn load_config() -> Result<Config> {
    // Try project-level config first
    if let Some(project_root) = find_project_root() {
        let project_config = project_root.join("git-helper.toml");
        if project_config.exists() {
            return load_config_from_path(&project_config);
        }
    }
    
    // Try global config
    if let Some(global_config) = get_global_config_path() {
        if global_config.exists() {
            return load_config_from_path(&global_config);
        }
    }
    
    // Return default config
    Ok(Config::default())
}

/// Get global config path
fn get_global_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("git-helper").join("config.toml"))
}

/// Load config from a specific path
fn load_config_from_path(path: &PathBuf) -> Result<Config> {
    let content = fs::read_to_string(path)
        .map_err(|e| anyhow!("Failed to read config file: {}", e))?;
    
    let config: Config = toml::from_str(&content)
        .map_err(|e| anyhow!("Failed to parse config file: {}", e))?;
    
    Ok(config)
}
