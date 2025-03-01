use actix_web::{test, App, web};
use andy::{routes, models::BlogPost};
use handlebars::Handlebars;
use tokio::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[actix_web::test]
async fn test_blog_post_creation_and_rendering() {
    // Create test content
    let content_dir = PathBuf::from("content");
    if !content_dir.exists() {
        fs::create_dir_all(&content_dir).await.unwrap();
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
    fs::write(&test_post_path, test_post_content).await.unwrap();

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
    fs::remove_file(test_post_path).await.unwrap_or_default();
}

#[actix_web::test]
async fn test_server_root_route() {
    // Create a test post for the homepage
    let content_dir = PathBuf::from("content");
    if !content_dir.exists() {
        fs::create_dir_all(&content_dir).await.unwrap();
    }

    let test_post_path = content_dir.join("test-post.md");
    let test_post_content = r#"---
title: Test Post
date: 2024-03-01T12:00:00Z
description: A test post
slug: test-post
---

Test post content.
"#;
    fs::write(&test_post_path, test_post_content).await.unwrap();

    // Initialize handlebars
    let mut handlebars = Handlebars::new();
    
    // Register formatDate helper
    handlebars.register_helper("formatDate", Box::new(|
        h: &handlebars::Helper,
        _: &handlebars::Handlebars,
        _: &handlebars::Context,
        _: &mut handlebars::RenderContext,
        out: &mut dyn handlebars::Output,
    | -> handlebars::HelperResult {
        let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
        
        if let Ok(date) = chrono::DateTime::parse_from_rfc3339(param) {
            let utc_date = date.with_timezone(&chrono::Utc);
            let formatted = utc_date.format("%B %e, %Y").to_string();
            out.write(&formatted)?;
        } else {
            out.write(param)?;
        }
        
        Ok(())
    }));
    
    // Register currentYear helper
    handlebars.register_helper("currentYear", Box::new(|
        _: &handlebars::Helper,
        _: &handlebars::Handlebars,
        _: &handlebars::Context,
        _: &mut handlebars::RenderContext,
        out: &mut dyn handlebars::Output,
    | -> handlebars::HelperResult {
        let current_year = chrono::Utc::now().format("%Y").to_string();
        out.write(&current_year)?;
        Ok(())
    }));

    // Register all templates
    handlebars
        .register_templates_directory(".hbs", "templates")
        .expect("Failed to register handlebars templates");
    
    let handlebars_ref = Arc::new(handlebars);
    
    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(handlebars_ref.clone()))
            .service(actix_files::Files::new("/static", "static"))
            .configure(routes::configure)
    ).await;
    
    // Send request to root page
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Check if the response is OK
    assert!(resp.status().is_success());
    
    // Get response body
    let body = test::read_body(resp).await;
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // Check if body contains expected elements
    assert!(body_str.contains("Welcome to My Blog"));
    assert!(body_str.contains("Latest Posts"));
    
    // Clean up
    fs::remove_file(test_post_path).await.unwrap_or_default();
}