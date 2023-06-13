use super::StoreOperations;
use std::collections::HashMap;
use std::io::Result;

pub struct StringStore {
    pub inner: HashMap<String, String>,
}

impl StoreOperations for StringStore {
    type StoreType = String;

    fn insert(&mut self, key: &str, value: Self::StoreType) -> Result<Self::StoreType> {
        self.inner.insert(key.to_string(), value.to_string());

        Ok(value)
    }

    fn retrieve(&self, key: &str) -> Result<Self::StoreType> {
        if let Some(value) = self.inner.get(key) {
            return Ok(value.clone());
        }

        Ok("".to_string())
    }

    fn remove(&mut self, key: &str) -> Result<Self::StoreType> {
        if let Some(value) = self.inner.remove(key) {
            return Ok(value);
        }

        Ok("".to_string())
    }

    fn clear(&mut self) {
        self.inner.clear();
    }
}
