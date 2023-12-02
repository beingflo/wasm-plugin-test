use anyhow::Result;
use extism_pdk::*;
use json::Value;
use serde::Serialize;

#[host_fn]
extern "ExtismHost" {
    fn query_db(key: String) -> Json<Value>;
}

#[derive(Serialize)]
pub struct DataPoint {
    co2: i64,
}

#[derive(Serialize)]
pub struct Data {
    date: String,
    data: Vec<DataPoint>,
}

#[plugin_fn]
pub fn count_vowels() -> FnResult<Json<Data>> {
    let data = Data {
        date: "test".to_owned(),
        data: vec![DataPoint { co2: 123 }],
    };

    Ok(Json(data))
}
