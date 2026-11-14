use std::{collections::HashMap, env::args, fs::read_to_string, io::Result};

fn read_env() -> Result<HashMap<String, String>> {
    let path = args().nth(1).unwrap_or_else(|| ".env".to_string());
    let content = read_to_string(path)?;
    let mut result: HashMap<String, String> = HashMap::new();

    for lines in content.lines() {
        if let Some((k, v)) = lines.split_once('=') {
            result.insert(k.trim().to_string(), v.trim().to_string());
        }
    }

    Ok(result)
}

pub fn var<'a>(name: &'a str) -> String {
    read_env().unwrap()[name].to_owned()
}
