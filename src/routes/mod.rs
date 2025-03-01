mod home;
mod about;
mod blog;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    home::configure(cfg);
    about::configure(cfg);
    blog::configure(cfg);
}