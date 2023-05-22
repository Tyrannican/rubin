use std::collections::HashMap;
use std::io;

pub struct Vault {
    pub strings: HashMap<String, String>,
}

impl Vault {
    pub fn empty() -> Self {
        Self {
            strings: HashMap::default(),
        }
    }

    pub fn insert_string(&mut self, key: &str, value: &str) -> io::Result<String> {
        println!("Inserting string: {key}");
        let _ = self.strings.insert(key.to_string(), value.to_string());

        Ok(value.to_string())
    }

    pub fn get_string(&self, key: &str) -> io::Result<String> {
        println!("Retrieving value for {}", key);
        if let Some(value) = self.strings.get(key) {
            return Ok(value.clone());
        }

        Ok("".to_string())
    }
}
