use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use rusqlite::Connection;
use serde::Serialize;
use serde_json::Value;
use tokio::sync::Mutex;

#[derive(Serialize, Debug)]
pub struct MetricRow {
    bucket: String,
    date: Option<String>,
    data: Value,
}

pub async fn metrics_handler(
    Path(bucket): Path<String>,
    connection: Extension<Arc<Mutex<Connection>>>,
) -> Json<Vec<MetricRow>> {
    let connection = connection.lock().await;
    let mut stmt = connection
        .prepare("SELECT * FROM metrics WHERE bucket = ?1")
        .unwrap();
    let mut rows = stmt.query([bucket]).unwrap();

    let mut metrics = vec![];
    while let Some(row) = rows.next().unwrap() {
        let m = MetricRow {
            bucket: row.get(0).unwrap(),
            date: row.get(1).unwrap(),
            data: row.get(2).unwrap(),
        };
        metrics.push(m);
    }

    Json(metrics)
}
