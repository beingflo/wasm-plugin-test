use extism_pdk::*;
use serde_json::Value;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MetricRow {
    date: String,
    data: Value,
}

#[plugin_fn]
pub fn clip(Json(mut data): Json<Vec<MetricRow>>) -> FnResult<Json<Vec<MetricRow>>> {
    for d in data.iter_mut() {
        let this_value = d.data["co2"].as_number().unwrap().as_i64().unwrap();

        if this_value > 690 && this_value < 800 {
            d.data["co2"] = Value::Null;
        }
    }
    Ok(Json(data))
}
