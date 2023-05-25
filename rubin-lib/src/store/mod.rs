pub mod persistence;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize)]
pub struct MemStore {
    #[serde(skip)]
    pub(crate) path: PathBuf,

    pub strings: HashMap<String, String>,
}

impl MemStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_string(&mut self, key: &str, value: &str) -> io::Result<String> {
        let _ = self.strings.insert(key.to_string(), value.to_string());

        Ok(value.to_string())
    }

    pub fn get_string(&self, key: &str) -> io::Result<String> {
        dbg!(format!("Retrieving value for {}", key));
        if let Some(value) = self.strings.get(key) {
            return Ok(value.clone());
        }

        Ok("".to_string())
    }
}
