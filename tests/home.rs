use actix_web::{test, App, web};
use andy::routes;
use handlebars::Handlebars;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[actix_web::test]
async fn test_home_route() {
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

This is a test post for testing the home route.
"#;
        fs::write(&test_post_path, test_post_content).unwrap();
    }

    // Initialize handlebars with templates
    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".hbs", "templates").expect("Failed to register templates");
    let handlebars_ref = Arc::new(handlebars);

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(handlebars_ref))
            .configure(routes::configure)
    ).await;
    
    // Send request to home page
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Check if the response is OK
    assert!(resp.status().is_success());
}