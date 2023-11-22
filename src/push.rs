use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension, Json};
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
pub struct PushMetric {
    date: Option<String>,
    data: Value,
}

pub async fn push_handler(
    Path(bucket): Path<String>,
    connection: Extension<Arc<Mutex<Connection>>>,
    Json(body): Json<PushMetric>,
) -> StatusCode {
    let result = connection.lock().await.execute(
        "INSERT INTO metrics (bucket, date, data) VALUES (?1, ?2, ?3)",
        (bucket, body.date.unwrap(), body.data.to_owned()),
    );

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
