mod metrics;
mod migration;

use std::sync::{Arc, Mutex};

use axum::{
    http::HeaderValue,
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use metrics::{bulk_insert_metrics, get_metrics, insert_metrics};
use migration::apply_migrations;
use rusqlite::Connection;
use tower_http::cors::{AllowHeaders, AllowMethods, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut conn = Connection::open("./db.sqlite")?;

    apply_migrations(&mut conn);

    let origin: String = dotenv::var("DOMAIN")
        .expect("DOMAIN env variable missing")
        .as_str()
        .parse()
        .expect("DOMAIN env variable malformed");

    let app = Router::new()
        .route("/metrics/:bucket", post(insert_metrics))
        .route("/metrics", post(bulk_insert_metrics))
        .route("/metrics/:bucket", get(get_metrics))
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
