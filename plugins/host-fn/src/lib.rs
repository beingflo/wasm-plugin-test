use anyhow::Result;
use extism_pdk::*;
use json::Value;
use serde::{Deserialize, Serialize};

#[host_fn]
extern "ExtismHost" {
    fn query_db(query: String) -> Json<Vec<Data>>;
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    date: String,
    data: Value,
}

#[plugin_fn]
pub fn host_fn() -> FnResult<Json<Vec<Data>>> {
    let result = unsafe {
        query_db("SELECT date, data FROM metrics WHERE bucket = ?1 ORDER BY date".to_owned())
    };

    let Json(mut data) = result?;

    for d in data.iter_mut() {
        let this_value = d.data["co2"].as_number().unwrap().as_i64().unwrap();

        if this_value > 750 && this_value < 800 {
            d.data["co2"] = Value::Null;
        }
    }

    Ok(Json(data))
}
