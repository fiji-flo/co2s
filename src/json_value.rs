use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use gtmpl::Value;
use serde_json;

pub fn json_to_value(y: serde_json::Value) -> Value {
    let v = match y {
        serde_json::Value::Array(a) => Value::Array(
            a.into_iter()
                .map(|y| json_to_value(y))
                .collect::<Vec<Value>>(),
        ),
        serde_json::Value::Bool(b) => b.into(),
        serde_json::Value::Object(h) => Value::Map(
            h.into_iter()
                .map(|(k, v)| (k.to_owned(), json_to_value(v)))
                .collect::<HashMap<String, Value>>(),
        ),
        serde_json::Value::Number(i) => {
            if let Some(i) = i.as_u64() {
                i.into()
            } else if let Some(i) = i.as_i64() {
                i.into()
            } else if let Some(i) = i.as_f64() {
                i.into()
            } else {
                Value::NoValue
            }
        }
        serde_json::Value::String(s) => s.into(),
        _ => Value::NoValue,
    };
    v
}

pub fn extend(org: &Value, upd: &Value) -> Value {
    println!("org:\n{}", org);
    println!("upd:\n{}", upd);
    match (org, upd) {
        (Value::Map(o), Value::Map(u)) => {
            println!("hit");
            let mut o_map: HashMap<String, Value> = o.clone();
            for (k, v) in u.iter() {
                let v = if o_map.contains_key(k) {
                    extend(o_map.get(k).unwrap(), v)
                } else {
                    v.clone()
                };
                o_map.insert(k.clone(), v);
            }
            o_map.into()
        }
        _ => upd.clone(),
    }
}

pub fn read_config(path: &Path) -> Result<Value, String> {
    let mut f = File::open(path).map_err(|e| format!("file not found: {}", e))?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .map_err(|e| format!("unable to read file: {}", e))?;
    let y: serde_json::Value = serde_json::from_str(&contents)
        .map_err(|e| format!("unable to load json: {}", e))?;
    Ok(json_to_value(y))
}

#[cfg(test)]
mod test {
    extern crate gtmpl_value;
    use super::*;

    #[test]
    fn some_json() {
        let ys = r#"{"some": [2000, "bar", false, 1.234]}"#;

        let y = serde_json::from_str(ys).unwrap();
        let v = json_to_value(y);
        assert_eq!(
            "{\"some\": Array([Number(Number { n: U(2000) }), String(\"bar\"), Bool(false), \
             Number(Number { n: F(1.234) })])}",
            &v.to_string()
        );
    }

    #[test]
    fn test_extend() {
        let a = r#"{"a": 1, "b": 2, "c": 3}"#;
        let b = r#"{"a": 1, "b": 23, "c": 3}"#;

        let y = serde_json::from_str(a).unwrap();
        let a = json_to_value(y);
        let y = serde_json::from_str(b).unwrap();
        let b = json_to_value(y);
        let out = extend(&a, &b);
        let out: HashMap<String, i64> = gtmpl_value::from_value(&out).unwrap();
        assert_eq!(out.get("b"), Some(&23i64));
    }

    #[test]
    fn test_nexted_extend() {
        let a = r#"{"a": 1, "b": {"a": 1, "b": 23}, "c": 3}"#;
        let b = r#"{"a": 1, "b": {"a": 1, "b": 42, "c": 3}, "c": 3}"#;

        let y = serde_json::from_str(a).unwrap();
        let a = json_to_value(y);
        let y = serde_json::from_str(b).unwrap();
        let b = json_to_value(y);
        let out = extend(&a, &b);
        assert!(!out.to_string().contains("U(23)"));
        assert!(out.to_string().contains("U(42)"));
    }
}
