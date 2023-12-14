use std::collections::VecDeque;

use extism_pdk::*;
use json::{Map, Number};
use serde_json::Value;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MetricRow {
    date: String,
    data: Value,
}

#[plugin_fn]
pub fn run(Json(data): Json<Vec<MetricRow>>) -> FnResult<Json<Vec<MetricRow>>> {
    let mut values = Vec::new();

    let mut temp_window = VecDeque::new();

    for d in data.into_iter() {
        let co2 = d.data["co2"].as_number().unwrap().as_i64().unwrap();
        let temperature = d.data["temperature"].as_number().unwrap().as_f64().unwrap();

        temp_window.push_back(temperature);

        if temp_window.len() > 5 {
            temp_window.pop_front();
        }

        let mut map = Map::new();

        map.insert("co2".to_owned(), Value::Number(Number::from(co2)));
        map.insert(
            "temperature".to_owned(),
            Value::Number(
                Number::from_f64(temp_window.iter().sum::<f64>() / temp_window.len() as f64)
                    .unwrap(),
            ),
        );

        values.push(MetricRow {
            date: d.date,
            data: Value::Object(map),
        });
    }

    Ok(Json(values))
}
