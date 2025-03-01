mod home;
mod about;
mod blog;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    // Home route - shows latest posts
    home::configure(cfg);
    
    // About page
    about::configure(cfg);
    
    // Archive page
    cfg.service(web::resource("/archive").route(web::get().to(blog::archive)));
    
    // Individual post pages
    cfg.service(web::resource("/post/{slug}").route(web::get().to(blog::post)));
}