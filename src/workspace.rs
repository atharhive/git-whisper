use anyhow::{Result, Context};
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoEntry {
    pub name: String,
    pub path: String,
    pub url: Option<String>,
    pub added_at: i64,
}

pub struct Workspace {
    config_path: PathBuf,
}

impl Workspace {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().context("Could not find home directory")?;
        let config_dir = home.join(".whisper");
        fs::create_dir_all(&config_dir)?;
        
        Ok(Self {
            config_path: config_dir.join("repos.json"),
        })
    }
    
    pub fn add_repo(&self, name: String, path: String, url: Option<String>) -> Result<()> {
        let mut repos = self.list_repos()?;
        
        let entry = RepoEntry {
            name,
            path,
            url,
            added_at: chrono::Utc::now().timestamp(),
        };
        
        repos.push(entry);
        self.save_repos(&repos)?;
        
        Ok(())
    }
    
    pub fn get_last_repo(&self) -> Result<Option<RepoEntry>> {
        let repos = self.list_repos()?;
        Ok(repos.last().cloned())
    }
    
    pub fn list_repos(&self) -> Result<Vec<RepoEntry>> {
        if !self.config_path.exists() {
            return Ok(Vec::new());
        }
        
        let content = fs::read_to_string(&self.config_path)?;
        let repos: Vec<RepoEntry> = serde_json::from_str(&content)?;
        Ok(repos)
    }
    
    fn save_repos(&self, repos: &[RepoEntry]) -> Result<()> {
        let content = serde_json::to_string_pretty(repos)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }
}
