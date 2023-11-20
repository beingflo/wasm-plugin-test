mod migration;
mod push;

use std::sync::Arc;

use axum::{
    http::HeaderValue,
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use migration::apply_migrations;
use push::push_handler;
use rusqlite::Connection;
use tokio::sync::Mutex;
use tower_http::cors::{AllowHeaders, AllowMethods, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut conn = Connection::open_in_memory()?;

    apply_migrations(&mut conn);

    let origin: String = dotenv::var("DOMAIN")
        .expect("DOMAIN env variable missing")
        .as_str()
        .parse()
        .expect("DOMAIN env variable malformed");

    let app = Router::new()
        .route("/push/:bucket", post(push_handler))
        .layer(Extension(Arc::new(Mutex::new(conn))))
        .layer(
            CorsLayer::new()
                .allow_origin(
                    HeaderValue::from_str(&origin).expect("DOMAIN env variable not valid"),
                )
                .allow_methods(AllowMethods::any())
                .allow_headers(AllowHeaders::any()),
        );

    axum::Server::bind(&"0.0.0.0:5005".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
