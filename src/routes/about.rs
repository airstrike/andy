use actix_web::{web, HttpResponse, Responder};
use handlebars::Handlebars;
use serde_json::json;
use std::sync::Arc;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/about").route(web::get().to(about)));
}

async fn about(hb: web::Data<Arc<Handlebars<'_>>>) -> impl Responder {
    let data = json!({
        "title": "About",
        "description": "About me and this website",
    });
    
    let body = hb.render("about", &data).unwrap_or_else(|err| {
        log::error!("Template rendering error: {}", err);
        "Error rendering template".to_string()
    });
    
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}