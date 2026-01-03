use anyhow::{Result, Context};
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub timestamp: i64,
    pub files_changed: Vec<FileChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub file_path: String,
    pub summary: String,
}

pub struct GitAnalyzer {
    repo: Repository,
}

impl GitAnalyzer {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::open(path)
            .context("Failed to open git repository. Make sure the path is a valid git repository.")?;
        
        Ok(Self { repo })
    }
    
    pub fn get_commits(&self) -> Result<Vec<Commit>> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TIME)?;
        
        let mut commits = Vec::new();
        
        for oid in revwalk {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;
            
            let hash = format!("{}", oid);
            let message = commit.message().unwrap_or("").to_string();
            let author = commit.author().name().unwrap_or("Unknown").to_string();
            let timestamp = commit.time().seconds();
            
            // Get file changes
            let files_changed = self.get_file_changes(&commit)?;
            
            commits.push(Commit {
                hash,
                message,
                author,
                timestamp,
                files_changed,
            });
        }
        
        Ok(commits)
    }
    
    fn get_file_changes(&self, commit: &git2::Commit) -> Result<Vec<FileChange>> {
        let mut files = Vec::new();
        
        let tree = commit.tree()?;
        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };
        
        let diff = self.repo.diff_tree_to_tree(
            parent_tree.as_ref(),
            Some(&tree),
            None,
        )?;
        
        diff.foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    let file_path = path.to_string_lossy().to_string();
                    let summary = format!("{:?}", delta.status());
                    
                    files.push(FileChange {
                        file_path,
                        summary,
                    });
                }
                true
            },
            None,
            None,
            None,
        )?;
        
        Ok(files)
    }
}
