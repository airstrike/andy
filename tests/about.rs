use actix_web::{test, App, web};
use andy::routes;

#[actix_web::test]
async fn test_about_route() {
    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(handlebars::Handlebars::new()))
            .configure(routes::configure)
    ).await;
    
    // Send request to about page
    let req = test::TestRequest::get().uri("/about").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Check if the response is OK
    assert!(resp.status().is_success());
}