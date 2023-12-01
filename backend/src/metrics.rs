use extism::*;
use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension, Json};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub struct MetricRow {
    date: Option<String>,
    data: Value,
}

pub async fn get_metrics(
    Path(bucket): Path<String>,
    connection: Extension<Arc<Mutex<Connection>>>,
) -> Json<Vec<MetricRow>> {
    let connection = connection.lock().await;
    let mut stmt = connection
        .prepare("SELECT date, data FROM metrics WHERE bucket = ?1")
        .unwrap();
    let mut rows = stmt.query([bucket]).unwrap();

    let mut metrics = vec![];
    while let Some(row) = rows.next().unwrap() {
        let m = MetricRow {
            date: row.get(0).unwrap(),
            data: row.get(1).unwrap(),
        };
        metrics.push(m);
    }

    let url = Wasm::file("./plugins/clip.wasm");
    let manifest = Manifest::new([url]);
    let mut plugin = Plugin::new(&manifest, [], true).unwrap();

    let extism::convert::Json(modified_metrics) = plugin
        .call::<extism::convert::Json<Vec<MetricRow>>, extism::convert::Json<Vec<MetricRow>>>(
            "clip",
            extism::convert::Json(metrics),
        )
        .unwrap();

    Json(modified_metrics)
}

#[derive(Deserialize, Debug)]
pub struct PushMetric {
    date: Option<String>,
    data: Value,
}

pub async fn insert_metrics(
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

#[derive(Deserialize, Debug)]
pub struct BulkPushMetric {
    bucket: String,
    date: Option<String>,
    data: Value,
}

pub async fn bulk_insert_metrics(
    connection: Extension<Arc<Mutex<Connection>>>,
    Json(body): Json<Vec<BulkPushMetric>>,
) -> StatusCode {
    for item in body.iter() {
        let result = connection.lock().await.execute(
            "INSERT INTO metrics (bucket, date, data) VALUES (?1, ?2, ?3)",
            (
                &item.bucket,
                &item.date.as_ref().unwrap(),
                item.data.to_owned(),
            ),
        );

        match result {
            Ok(_) => {}
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    StatusCode::OK
}
