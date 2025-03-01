use actix_web::{test, App, web};
use andy::routes;
use handlebars::Handlebars;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[actix_web::test]
async fn test_archive_route() {
    // Ensure content dir exists and has at least one file
    let content_dir = PathBuf::from("content");
    if !content_dir.exists() {
        fs::create_dir_all(&content_dir).unwrap();
    }

    // Create a test post if none exists
    let test_post_path = content_dir.join("test-archive.md");
    if !fs::metadata(&test_post_path).is_ok() {
        let test_post_content = r#"---
title: Test Archive Post
date: 2024-03-01T12:00:00Z
description: A test post for testing the archive
slug: test-archive-post
---

# Test Archive Content

This is a test post for testing the archive route.
"#;
        fs::write(&test_post_path, test_post_content).unwrap();
    }

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

    // Register templates
    handlebars.register_templates_directory(".hbs", "templates").expect("Failed to register templates");
    let handlebars_ref = Arc::new(handlebars);

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(handlebars_ref))
            .service(actix_files::Files::new("/static", "static"))
            .configure(routes::configure)
    ).await;
    
    // Send request to archive page
    let req = test::TestRequest::get().uri("/archive").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Check if the response is OK
    assert!(resp.status().is_success());
    
    // Get response body
    let body = test::read_body(resp).await;
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // Check if body contains expected elements
    assert!(body_str.contains("Blog Archive"));
    assert!(body_str.contains("test-archive-post"));
    assert!(body_str.contains("Test Archive Post"));
}

#[actix_web::test]
async fn test_post_route() {
    // Ensure content dir exists and has at least one file
    let content_dir = PathBuf::from("content");
    if !content_dir.exists() {
        fs::create_dir_all(&content_dir).unwrap();
    }

    // Create a test post if none exists
    let test_post_path = content_dir.join("test-post.md");
    if !fs::metadata(&test_post_path).is_ok() {
        let test_post_content = r#"---
title: Test Post
date: 2024-03-01T12:00:00Z
description: A test post for testing
slug: test-post
---

# Test Post Content

This is a test post for testing the post route.
"#;
        fs::write(&test_post_path, test_post_content).unwrap();
    }

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

    // Register templates
    handlebars.register_templates_directory(".hbs", "templates").expect("Failed to register templates");
    let handlebars_ref = Arc::new(handlebars);

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(handlebars_ref))
            .service(actix_files::Files::new("/static", "static"))
            .configure(routes::configure)
    ).await;
    
    // Send request to post page
    let req = test::TestRequest::get().uri("/post/test-post").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Check if the response is OK
    assert!(resp.status().is_success());
    
    // Get response body
    let body = test::read_body(resp).await;
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // Check if body contains expected elements
    assert!(body_str.contains("Test Post"));
    assert!(body_str.contains("Test Post Content"));
    assert!(body_str.contains("March  1, 2024"));
}