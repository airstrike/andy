use actix_web::{test, App, web};
use andy::{routes, models::BlogPost};
use handlebars::Handlebars;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[actix_web::test]
async fn test_blog_post_creation_and_rendering() {
    // Create test content
    let content_dir = PathBuf::from("content");
    if !content_dir.exists() {
        fs::create_dir_all(&content_dir).unwrap();
    }

    // Create a test post with code blocks for syntax highlighting
    let test_post_path = content_dir.join("syntax-test.md");
    let test_post_content = r#"---
title: Syntax Test Post
date: 2024-03-01T15:00:00Z
description: Testing syntax highlighting
slug: syntax-test
---

# Testing Syntax Highlighting

Here's a Rust code block:

```rust
fn main() {
    println!("Hello, world!");
}
```

And JavaScript:

```javascript
console.log("Hello, world!");
```
"#;
    fs::write(&test_post_path, test_post_content).unwrap();

    // Test loading the blog post
    let posts = BlogPost::get_all_posts().await.expect("Failed to load blog posts");
    let test_post = posts.iter().find(|p| p.slug == "syntax-test").expect("Test post not found");
    
    // Verify the post has the correct content
    assert_eq!(test_post.title, "Syntax Test Post");
    assert_eq!(test_post.description, "Testing syntax highlighting");
    
    // Verify the HTML contains syntax highlighting elements
    assert!(test_post.html_content.contains("<pre"));
    assert!(test_post.html_content.contains("code-rust"));
    assert!(test_post.html_content.contains("code-javascript"));
    
    // Initialize handlebars with templates
    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".hbs", "templates").expect("Failed to register templates");
    let handlebars_ref = Arc::new(handlebars);

    // Create test app and test accessing the post
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(handlebars_ref))
            .configure(routes::configure)
    ).await;
    
    // Send request to post page
    let req = test::TestRequest::get().uri("/post/syntax-test").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Check if the response is OK
    assert!(resp.status().is_success());
    
    // Clean up - remove test post
    fs::remove_file(test_post_path).unwrap_or_default();
}