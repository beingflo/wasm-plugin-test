mod push;

use axum::{http::HeaderValue, routing::post, Router};
use dotenv::dotenv;
use push::push_handler;
use tower_http::cors::{AllowHeaders, AllowMethods, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let origin: String = dotenv::var("DOMAIN")
        .expect("DOMAIN env variable missing")
        .as_str()
        .parse()
        .expect("DOMAIN env variable malformed");

    let app = Router::new().route("/push", post(push_handler)).layer(
        CorsLayer::new()
            .allow_origin(HeaderValue::from_str(&origin).expect("DOMAIN env variable not valid"))
            .allow_methods(AllowMethods::any())
            .allow_headers(AllowHeaders::any()),
    );

    axum::Server::bind(&"0.0.0.0:5005".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
