use actix_web::{web, App, HttpServer, middleware};
use actix_files as fs;
use handlebars::Handlebars;
use std::sync::Arc;
use chrono::{DateTime, Utc};

use andy::routes;

// Helper function for formatting dates in Handlebars templates
fn format_date_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    // Get the first parameter as a string
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    
    // Parse the date string
    if let Ok(date) = DateTime::parse_from_rfc3339(param) {
        let utc_date = date.with_timezone(&Utc);
        // Format date: March 1, 2024
        let formatted = utc_date.format("%B %e, %Y").to_string();
        out.write(&formatted)?;
    } else {
        out.write(param)?;
    }
    
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting server at http://localhost:8080");

    // Initialize handlebars
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("formatDate", Box::new(format_date_helper));
    
    // Add currentYear helper
    handlebars.register_helper("currentYear", Box::new(|
        _: &handlebars::Helper,
        _: &handlebars::Handlebars,
        _: &handlebars::Context,
        _: &mut handlebars::RenderContext,
        out: &mut dyn handlebars::Output,
    | -> handlebars::HelperResult {
        let current_year = Utc::now().format("%Y").to_string();
        out.write(&current_year)?;
        Ok(())
    }));
    
    handlebars
        .register_templates_directory(".hbs", "templates")
        .expect("Failed to register handlebars templates");
    let handlebars_ref = Arc::new(handlebars);

    // Start http server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(handlebars_ref.clone()))
            .wrap(middleware::Logger::default())
            // Static files
            .service(fs::Files::new("/static", "static"))
            // Routes
            .configure(routes::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
