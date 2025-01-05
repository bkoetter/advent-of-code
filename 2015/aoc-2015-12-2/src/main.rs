use serde_json::Value;
use std::error::Error;

fn extract_json(count: i64, json: &Value) -> i64 {
    match json {
        Value::Object(o) if o.values().any(|v| v == "red") => count,
        Value::Object(o) => o.iter().map(|v| extract_json(count, v.1)).sum(),
        Value::Array(a) => a.iter().map(|v| extract_json(count, v)).sum(),
        Value::Number(n) => count + n.as_i64().unwrap(),
        _ => count,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let json = &serde_json::from_slice(include_bytes!("../input.json"))?;
    println!("{}", extract_json(0, json));
    Ok(())
}
