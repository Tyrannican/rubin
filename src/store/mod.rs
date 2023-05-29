pub mod persistence;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::io;

#[derive(Default, Serialize, Deserialize)]
pub struct MemStore {
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
        if let Some(value) = self.strings.get(key) {
            return Ok(value.clone());
        }

        Ok("".to_string())
    }
}

#[cfg(test)]
mod memstore {
    use super::*;

    #[test]
    fn empty() {
        let ms = MemStore::new();
        assert_eq!(ms.strings.len(), 0);
    }

    #[test]
    fn add_entries() -> io::Result<()> {
        let mut ms = MemStore::new();
        let mut result = ms.insert_string("key1", "value1")?;

        assert_eq!(ms.strings.len(), 1);
        assert_eq!(result, "value1");

        result = ms.insert_string("key2", "value2")?;

        assert_eq!(ms.strings.len(), 2);
        assert_eq!(result, "value2");

        Ok(())
    }

    #[test]
    fn add_loads_of_entries() -> io::Result<()> {
        let mut ms = MemStore::new();

        for i in 0..100_000 {
            let key = format!("key-{}", i);
            let value = format!("value-{}", i);
            let result = ms.insert_string(&key, &value)?;
            assert_eq!(result, value);
        }

        Ok(())
    }

    #[test]
    fn get_entries() -> io::Result<()> {
        let mut ms = MemStore::new();
        ms.insert_string("key1", "value1")?;
        ms.insert_string("key2", "value2")?;

        let mut result = ms.get_string("key2")?;
        assert_eq!(result, "value2");
        result = ms.get_string("key1")?;
        assert_eq!(result, "value1");

        Ok(())
    }

    #[test]
    fn no_entries() -> io::Result<()> {
        let ms = MemStore::new();
        let result = ms.get_string("key1")?;
        assert_eq!(result, "");

        Ok(())
    }
}
