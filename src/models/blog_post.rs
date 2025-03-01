use chrono::{DateTime, Utc};
use pulldown_cmark::{html, Event, Parser, Tag};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::OnceLock;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::{SyntaxReference, SyntaxSet};
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
    
    #[error("Syntax highlighting error: {0}")]
    SyntaxHighlighting(String),
}

// Static globals for syntax highlighting
static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();
static THEME: OnceLock<Theme> = OnceLock::new();

// Helper to get syntax set
fn get_syntax_set() -> &'static SyntaxSet {
    SYNTAX_SET.get_or_init(|| SyntaxSet::load_defaults_newlines())
}

// Helper to get theme
fn get_theme() -> &'static Theme {
    THEME.get_or_init(|| {
        let theme_set = THEME_SET.get_or_init(ThemeSet::load_defaults);
        // Use the base16-ocean-dark theme
        theme_set.themes["base16-ocean.dark"].clone()
    })
}

// Helper to find syntax definition for a language
fn find_syntax(lang: &str) -> Option<&'static SyntaxReference> {
    let syntax_set = get_syntax_set();
    syntax_set.find_syntax_by_token(lang)
        .or_else(|| syntax_set.find_syntax_by_extension(lang))
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
        
        // Convert markdown to HTML with syntax highlighting
        let parser = Parser::new(&markdown_content);
        let mut html_output = String::new();
        
        // Process events, adding syntax highlighting for code blocks
        let mut code_block_content = String::new();
        let mut code_block_lang = String::new();
        let mut in_code_block = false;
        
        let events: Vec<_> = parser.collect();
        let mut processed_events = Vec::new();
        
        for event in events.into_iter() {
            match event {
                Event::Start(Tag::CodeBlock(lang)) => {
                    in_code_block = true;
                    if let pulldown_cmark::CodeBlockKind::Fenced(name) = lang {
                        code_block_lang = name.to_string();
                    } else {
                        code_block_lang = String::new();
                    }
                    code_block_content.clear();
                },
                Event::End(Tag::CodeBlock(_)) => {
                    in_code_block = false;
                    
                    // Apply syntax highlighting
                    if !code_block_lang.is_empty() {
                        if let Some(syntax) = find_syntax(&code_block_lang) {
                            match highlighted_html_for_string(
                                &code_block_content,
                                get_syntax_set(),
                                syntax,
                                get_theme()
                            ) {
                                Ok(highlighted_html) => {
                                    // Replace the code block with highlighted HTML
                                    processed_events.push(Event::Html(format!(
                                        "<pre class=\"code-block code-{}\"><code>{}</code></pre>",
                                        code_block_lang,
                                        highlighted_html
                                    ).into()));
                                    continue;
                                },
                                Err(e) => {
                                    log::warn!("Failed to highlight code: {}", e);
                                }
                            }
                        }
                    }
                    
                    // If no syntax highlighting was applied, fall back to regular code block
                    let kind = if !code_block_lang.is_empty() {
                        pulldown_cmark::CodeBlockKind::Fenced(code_block_lang.clone().into())
                    } else {
                        pulldown_cmark::CodeBlockKind::Indented
                    };
                    processed_events.push(Event::Start(Tag::CodeBlock(kind.clone())));
                    processed_events.push(Event::Text(code_block_content.clone().into()));
                    processed_events.push(Event::End(Tag::CodeBlock(kind)));
                },
                Event::Text(text) if in_code_block => {
                    code_block_content.push_str(&text);
                    continue;
                },
                _ => processed_events.push(event),
            }
        }
        
        html::push_html(&mut html_output, processed_events.into_iter());
        
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