use serde_json::Value;
use std::error::Error;

fn extract_json(count: i64, json: &Value) -> i64 {
    match &json {
        Value::Number(n) => count + n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(|a| extract_json(count, a)).sum(),
        Value::Object(o) => o.iter().map(|o| extract_json(count, o.1)).sum(),
        _ => count,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let count = 0;
    let json = &serde_json::from_slice(include_bytes!("../input.json"))?;
    println!("{:?}", extract_json(count, json));
    Ok(())
}
