use serde_json::Value;
use std::fs::File;

use crate::Args;

fn flatten_object(value: &Value) -> Value {
    let mut new_data = Value::Object(serde_json::Map::new());

    if let Some(data) = value.as_object() {
        // Loop through the keys
        for (k, v) in data {
            log::debug!("{}: {:?}", k, v);

            // If the value is an object, loop through the keys
            if v.is_object() {
                let flattened = flatten_object(v.into());

                let old_key = k.clone();
                for (k, v) in flattened.as_object().unwrap() {
                    let new_key = format!("{}.{}", old_key, k);
                    new_data[new_key] = v.clone();
                }
            } else {
                new_data[k] = v.clone();
            }
        }
    }

    new_data
}

pub fn flat(args: &Args) -> Result<Value, Box<dyn std::error::Error>> {
    let file = File::open(&args.input)?;

    let data: Value = serde_json::from_reader(file)?;

    log::debug!("{:?}", data);

    let flattened = flatten_object(&data);

    Ok(flattened)
}

pub fn unflat(args: &Args) -> Result<Value, Box<dyn std::error::Error>> {
    let file = File::open(&args.input)?;

    let data: Value = serde_json::from_reader(file)?;

    log::debug!("{:?}", data);

    let mut new_data = Value::Object(serde_json::Map::new());

    if let Some(data) = data.as_object() {
        for (k, v) in data {
            log::debug!("{}: {:?}", k, v);
            let splitted_key: Vec<&str> = k.split('.').collect();
            log::debug!("{:?}", splitted_key);

            let mut current: &mut Value = &mut new_data;

            // Loop through the keys
            for (i, key) in splitted_key.iter().enumerate() {
                log::debug!("{}: {}", i, key);

                if i == splitted_key.len() - 1 {
                    // If we're at the last key, set the value
                    log::debug!("{}: {}", key, v);
                    current[key] = v.clone();
                } else {
                    // If we're not at the last key, create a new object
                    if current[key].is_null() {
                        current[key] = Value::Object(serde_json::Map::new());
                    }

                    current = current.get_mut(key).unwrap();
                }
            }
        }
    }

    Ok(new_data)
}

/// Return the output in stdout or a file depending on the args
pub fn write_output(args: &Args, data: &Value) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(output) = &args.output {
        let mut file = File::create(output)?;
        serde_json::to_writer_pretty(&mut file, data)?;
    } else {
        serde_json::to_writer_pretty(std::io::stdout(), data)?;
        println!("\n");
    }

    Ok(())
}
