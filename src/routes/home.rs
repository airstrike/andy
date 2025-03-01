use actix_web::{web, HttpResponse, Result};
use handlebars::Handlebars;
use serde_json::json;
use std::sync::Arc;

use crate::models::BlogPost;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)));
}

async fn index(hb: web::Data<Arc<Handlebars<'_>>>) -> Result<HttpResponse> {
    let posts = match BlogPost::get_all_posts().await {
        Ok(posts) => posts,
        Err(e) => {
            log::error!("Failed to get blog posts: {}", e);
            return Ok(HttpResponse::InternalServerError().body("Failed to load blog posts"));
        }
    };
    
    // Take only the latest few posts for the home page
    let latest_posts = posts.into_iter().take(5).collect::<Vec<_>>();
    
    let data = json!({
        "title": "Blog",
        "description": "Personal blog and website",
        "posts": latest_posts,
        "isHome": true,
    });
    
    let body = hb.render("index", &data).unwrap_or_else(|err| {
        log::error!("Template rendering error: {}", err);
        "Error rendering template".to_string()
    });
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}