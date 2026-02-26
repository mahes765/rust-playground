use std::fs;
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use chrono::Local;

use crate::config::load_config;
use crate::utils::fs::find_project_root;

/// Template context for placeholder replacement
pub struct TemplateContext {
    pub message: String,
    pub branch: String,
    pub date: String,
    pub commit_type: String,
}

impl TemplateContext {
    pub fn new(message: &str, branch: &str, commit_type: &str) -> Self {
        TemplateContext {
            message: message.to_string(),
            branch: branch.to_string(),
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            commit_type: commit_type.to_string(),
        }
    }
}

/// Load template for a given commit type
/// Priority: project template > global template > config template > default
pub fn load_template(commit_type: &str) -> Result<String> {
    // Try project-level template
    if let Some(project_root) = find_project_root() {
        let template_path = project_root
            .join(".git-helper")
            .join("templates")
            .join(format!("{}.txt", commit_type));
        
        if template_path.exists() {
            return fs::read_to_string(&template_path)
                .map_err(|e| anyhow!("Failed to read project template: {}", e));
        }
    }
    
    // Try global template
    if let Some(global_template) = get_global_template_path(commit_type) {
        if global_template.exists() {
            return fs::read_to_string(&global_template)
                .map_err(|e| anyhow!("Failed to read global template: {}", e));
        }
    }
    
    // Try config-based template
    let config = load_config()?;
    if let Some(template) = config.get_commit_template(commit_type) {
        return Ok(template.clone());
    }
    
    // Default template
    Ok(format!("{}: {{message}}\n\nbranch: {{branch}}\ndate: {{date}}", commit_type))
}

/// Get global template path
fn get_global_template_path(commit_type: &str) -> Option<PathBuf> {
    dirs::config_dir().map(|p| {
        p.join("git-helper")
            .join("templates")
            .join(format!("{}.txt", commit_type))
    })
}

/// Fill template with context values
pub fn fill_template(template: &str, context: &TemplateContext) -> String {
    template
        .replace("{message}", &context.message)
        .replace("{branch}", &context.branch)
        .replace("{date}", &context.date)
        .replace("{type}", &context.commit_type)
}

/// Load and fill template in one step
pub fn render_commit_message(commit_type: &str, message: &str, branch: &str) -> Result<String> {
    let template = load_template(commit_type)?;
    let context = TemplateContext::new(message, branch, commit_type);
    Ok(fill_template(&template, &context))
}
