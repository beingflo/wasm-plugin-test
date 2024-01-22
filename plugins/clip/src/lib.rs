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
    for d in data.iter_mut() {
        if d.data.co2 > 850 {
            d.data.co2 *= 2;
        }
    }
    Ok(Json(data))
}
