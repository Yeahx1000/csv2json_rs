// file for file conversion logic
use crate::error::AppResult;
use serde_json::{Map, Value};
use std::io::Read;

pub fn csv_to_json(reader: impl Read) -> AppResult<Vec<Value>> {
    let mut csv_reader = csv::Reader::from_reader(reader);
    let headers = csv_reader.headers()?.clone();
    let mut records_json = Vec::new();

    for result in csv_reader.records() {
        let record = result?;
        let mut row_obj = Map::with_capacity(headers.len());

        for (header, value) in headers.iter().zip(record.iter()) {
            row_obj.insert(header.to_string(), Value::String(value.to_string()));
        }

        records_json.push(Value::Object(row_obj));
    }

    Ok(records_json)
}
