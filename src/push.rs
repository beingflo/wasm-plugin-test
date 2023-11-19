use axum::{extract::Path, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct PushMetric {
    date: Option<String>,
    data: Value,
}

pub async fn push_handler(Path(bucket): Path<String>, Json(body): Json<PushMetric>) -> StatusCode {
    println!("{} {:?}", bucket, body);
    return StatusCode::OK;
}
