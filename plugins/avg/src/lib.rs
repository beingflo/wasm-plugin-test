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

#[derive(serde::Serialize)]
pub struct ModifiedData {
    metrics: Vec<Row>,
    avg_temperature: f64,
}

#[plugin_fn]
pub fn run(Json(mut data): Json<Vec<Row>>) -> FnResult<Json<ModifiedData>> {
    let mut window = VecDeque::new();

    for d in data.iter_mut() {
        window.push_back(d.data.temperature);

        if window.len() > 1 {
            window.pop_front();
        }

        d.data.temperature = window.iter().sum::<f64>() / window.len() as f64;
    }

    let avg_temperature = data.iter().map(|d| d.data.temperature).sum::<f64>() / data.len() as f64 ;

    Ok(Json(ModifiedData { metrics: data, avg_temperature }))
}
