use actix_web::{web, HttpResponse, Result};
use handlebars::Handlebars;
use serde_json::json;
use std::sync::Arc;

use crate::models::BlogPost;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/blog")
            .route("", web::get().to(index))
            .route("/archive", web::get().to(archive))
            .route("/{slug}", web::get().to(post))
    );
}

async fn index(hb: web::Data<Arc<Handlebars<'_>>>) -> Result<HttpResponse> {
    let posts = match BlogPost::get_all_posts().await {
        Ok(posts) => posts,
        Err(e) => {
            log::error!("Failed to get blog posts: {}", e);
            return Ok(HttpResponse::InternalServerError().body("Failed to load blog posts"));
        }
    };
    
    // Only take the latest few posts for the index page
    let latest_posts = posts.into_iter().take(5).collect::<Vec<_>>();
    
    let data = json!({
        "title": "Blog",
        "description": "Latest blog posts",
        "posts": latest_posts,
    });
    
    let body = hb.render("blog/index", &data).unwrap_or_else(|err| {
        log::error!("Template rendering error: {}", err);
        "Error rendering template".to_string()
    });
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

async fn archive(hb: web::Data<Arc<Handlebars<'_>>>) -> Result<HttpResponse> {
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
    });
    
    let body = hb.render("blog/archive", &data).unwrap_or_else(|err| {
        log::error!("Template rendering error: {}", err);
        "Error rendering template".to_string()
    });
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

async fn post(
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