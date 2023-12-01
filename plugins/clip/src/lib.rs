use extism_pdk::*;
use json::Number;
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

        if this_value > 850 {
            d.data["co2"] = Value::from(Number::from(0));
        }
    }
    Ok(Json(data))
}
