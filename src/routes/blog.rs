use actix_web::{web, HttpResponse, Result};
use handlebars::Handlebars;
use serde_json::json;
use std::sync::Arc;

use crate::models::BlogPost;

// No longer using the configure function since we're calling these handlers directly now
// from the main routes module

pub async fn archive(hb: web::Data<Arc<Handlebars<'_>>>) -> Result<HttpResponse> {
    let posts = match BlogPost::get_all_posts().await {
        Ok(posts) => posts,
        Err(e) => {
            log::error!("Failed to get blog posts: {}", e);
            return Ok(HttpResponse::InternalServerError().body("Failed to load blog posts"));
        }
    };
    
    let data = json!({
        "title": "Archive",
        "description": "Archive of all blog posts",
        "posts": posts,
        "isArchive": true,
    });
    
    let body = hb.render("blog/archive", &data).unwrap_or_else(|err| {
        log::error!("Template rendering error: {}", err);
        "Error rendering template".to_string()
    });
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

pub async fn post(
    hb: web::Data<Arc<Handlebars<'_>>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let slug = path.into_inner();
    
    let posts = match BlogPost::get_all_posts().await {
        Ok(posts) => posts,
        Err(e) => {
            log::error!("Failed to get blog posts: {}", e);
            return Ok(HttpResponse::InternalServerError().body("Failed to load blog posts"));
        }
    };
    
    // Find the post with the matching slug
    let post = posts.into_iter().find(|p| p.slug == slug);
    
    match post {
        Some(post) => {
            let data = json!({
                "title": post.title,
                "description": post.description,
                "post": post,
            });
            
            let body = hb.render("blog/post", &data).unwrap_or_else(|err| {
                log::error!("Template rendering error: {}", err);
                "Error rendering template".to_string()
            });
            
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        },
        None => Ok(HttpResponse::NotFound().body("Post not found")),
    }
}