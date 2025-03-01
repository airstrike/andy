use chrono::{DateTime, Utc};
use pulldown_cmark::{html, Parser};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;
use thiserror::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlogPost {
    pub slug: String,
    pub title: String,
    pub date: DateTime<Utc>,
    pub description: String,
    pub content: String,
    pub html_content: String,
}

#[derive(Error, Debug)]
pub enum BlogPostError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Failed to parse frontmatter: {0}")]
    FrontmatterParse(String),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
}

impl BlogPost {
    pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, BlogPostError> {
        let content = fs::read_to_string(path).await?;
        
        // Simple frontmatter parsing (assumes ---yaml--- format)
        let parts: Vec<&str> = content.split("---").collect();
        if parts.len() < 3 {
            return Err(BlogPostError::FrontmatterParse("Invalid frontmatter format".into()));
        }
        
        let frontmatter = parts[1];
        let markdown_content = parts[2..].join("---");
        
        // Parse frontmatter (simplified)
        let mut title = None;
        let mut date = None;
        let mut description = None;
        let mut slug = None;
        
        for line in frontmatter.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() != 2 {
                continue;
            }
            
            let key = parts[0].trim();
            let value = parts[1].trim();
            
            match key {
                "title" => title = Some(value.to_string()),
                "date" => {
                    date = Some(DateTime::parse_from_rfc3339(value)
                        .map_err(|e| BlogPostError::FrontmatterParse(format!("Invalid date: {}", e)))?
                        .with_timezone(&Utc))
                },
                "description" => description = Some(value.to_string()),
                "slug" => slug = Some(value.to_string()),
                _ => {}
            }
        }
        
        let title = title.ok_or_else(|| BlogPostError::MissingField("title".into()))?;
        let date = date.ok_or_else(|| BlogPostError::MissingField("date".into()))?;
        let description = description.ok_or_else(|| BlogPostError::MissingField("description".into()))?;
        let slug = slug.ok_or_else(|| BlogPostError::MissingField("slug".into()))?;
        
        // Convert markdown to HTML
        let parser = Parser::new(&markdown_content);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        
        Ok(BlogPost {
            slug,
            title,
            date,
            description,
            content: markdown_content,
            html_content: html_output,
        })
    }
    
    pub async fn get_all_posts() -> Result<Vec<BlogPost>, BlogPostError> {
        let content_dir = "content";
        let mut entries = fs::read_dir(content_dir).await?;
        let mut posts = Vec::new();
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                match Self::from_file(&path).await {
                    Ok(post) => posts.push(post),
                    Err(e) => log::error!("Failed to parse blog post {:?}: {}", path, e),
                }
            }
        }
        
        // Sort by date, newest first
        posts.sort_by(|a, b| b.date.cmp(&a.date));
        
        Ok(posts)
    }
}