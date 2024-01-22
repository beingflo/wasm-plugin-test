use std::collections::VecDeque;
use extism_pdk::*;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DataPoint {
    co2: i64,
    temperature: f64,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Row {
    date: String,
    data: DataPoint,
}

#[plugin_fn]
pub fn run(Json(mut data): Json<Vec<Row>>) -> FnResult<Json<Vec<Row>>> {
    let mut window = VecDeque::new();

    for d in data.iter_mut() {
        let temperature = d.data.temperature;

        window.push_back(temperature);

        if window.len() > 1 {
            window.pop_front();
        }

        d.data.temperature = window.iter().sum::<f64>() / window.len() as f64;
    }

    Ok(Json(data))
}
