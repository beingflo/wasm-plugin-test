use extism::*;
use std::sync::{Arc, Mutex};

use axum::{extract::Path, http::StatusCode, Extension, Json};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct MetricRow {
    date: Option<String>,
    data: Value,
}

host_fn!(query_db(connection: Arc<Mutex<Connection>>; query: String) -> extism::Json<Vec<Data>> {
    let user_connection = connection.get()?;
    let locked_connection = user_connection.lock().unwrap();
    let connection = locked_connection.lock().unwrap();

    let mut stmt = connection
        .prepare(&query)
        .unwrap();
    let mut rows = stmt.query(["living_room".to_owned()]).unwrap();

    let mut metrics = vec![];
    while let Some(row) = rows.next().unwrap() {
        let m = MetricRow {
            date: row.get(0).unwrap(),
            data: row.get(1).unwrap(),
        };
        metrics.push(m);
    }
    Ok(extism::convert::Json(metrics))
});

#[derive(Serialize, Deserialize)]
pub struct Data {
    date: String,
    data: Value,
}

pub async fn get_metrics(
    Path(bucket): Path<String>,
    Extension(connection): Extension<Arc<Mutex<Connection>>>,
) -> Json<Vec<MetricRow>> {
    let metrics = {
        let connection = connection.lock().unwrap();
        let mut stmt = connection
            .prepare("SELECT date, data FROM metrics WHERE bucket = ?1 ORDER BY date")
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
        metrics
    };

    let user_data = UserData::new(connection.clone());

    let file = Wasm::file("./plugins/host_fn.wasm");
    let manifest = Manifest::new([file]);
    let mut plugin = PluginBuilder::new(manifest)
        .with_wasi(true)
        .with_function("query_db", [PTR], [PTR], user_data.clone(), query_db)
        .build()
        .unwrap();
    //let mut plugin = Plugin::new(&manifest, [], true).unwrap();

    let extism::convert::Json(modified_metrics) = plugin
        .call::<extism::convert::Json<Vec<MetricRow>>, extism::convert::Json<Vec<MetricRow>>>(
            "host_fn",
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
    let result = connection.lock().unwrap().execute(
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
        let result = connection.lock().unwrap().execute(
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
