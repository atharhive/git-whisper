use anyhow::{Result, Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::repository::Commit;

const GEMINI_API_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent";

#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Debug, Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize)]
struct Part {
    text: String,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Debug, Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Debug, Deserialize)]
struct ResponsePart {
    text: String,
}

pub struct GeminiClient {
    api_key: String,
    client: Client,
}

impl GeminiClient {
    pub fn new(api_key: &str) -> Result<Self> {
        Ok(Self {
            api_key: api_key.to_string(),
            client: Client::new(),
        })
    }
    
    pub async fn test_connection(&self) -> Result<()> {
        let url = format!("{}?key={}", GEMINI_API_URL, self.api_key);
        
        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: "Hello".to_string(),
                }],
            }],
        };
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to connect to Gemini API")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Gemini API error: {}", error_text);
        }
        
        Ok(())
    }
    
    pub async fn generate_project_summary(&self, commits: &[Commit]) -> Result<String> {
        if commits.is_empty() {
            return Ok("No commits provided to generate a summary.".to_string());
        }
        
        let commit_history = self.format_commits(commits);
        
        let prompt = format!(
            "Given the following git commit history, generate a concise, plain-English project summary. \
             Explain what problem the project tried to solve, how it evolved, and what actually matters. \
             Focus on intent and evolution, not implementation details. The summary should be similar to the example:\n\n\
             \"This project evolved from an initial scaffold into a functional application with authentication, \
             performance optimizations, and a refined developer experience.\"\n\n\
             {}", 
            commit_history
        );
        
        self.generate_content(&prompt).await
    }
    
    #[allow(dead_code)]
    pub async fn generate_changelog(&self, commits: &[Commit]) -> Result<String> {
        if commits.is_empty() {
            return Ok("No commits provided to generate a changelog.".to_string());
        }
        
        let commit_history = self.format_commits(commits);
        
        let prompt = format!(
            "Given the following git commit history, generate a clean, readable CHANGELOG draft. \
             Group related changes and highlight key features, fixes, and improvements. \
             The changelog should be similar to the example:\n\n\
             \"Added JWT-based authentication and refactored middleware to support scaling.\"\n\n\
             {}",
            commit_history
        );
        
        self.generate_content(&prompt).await
    }
    
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let url = format!("{}?key={}", GEMINI_API_URL, self.api_key);
        
        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
        };
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Gemini API error: {}", error_text);
        }
        
        let gemini_response: GeminiResponse = response.json().await?;
        
        let text = gemini_response
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .unwrap_or_else(|| "No response generated".to_string());
        
        Ok(text)
    }
    
    fn format_commits(&self, commits: &[Commit]) -> String {
        let mut output = String::from("Commit History:\n");
        
        for commit in commits {
            output.push_str(&format!("- {}: {}\n", commit.hash, commit.message));
            for file in &commit.files_changed {
                output.push_str(&format!("  - {} ({})\n", file.file_path, file.summary));
            }
        }
        
        output
    }
}
