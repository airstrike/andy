use actix_web::{test, App, web};
use andy::routes;
use handlebars::Handlebars;
use std::sync::Arc;

#[actix_web::test]
async fn test_about_route() {
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
    
    // Send request to about page
    let req = test::TestRequest::get().uri("/about").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Check if the response is OK
    assert!(resp.status().is_success());
}