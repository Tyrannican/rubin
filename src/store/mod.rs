//! In-memory and persistent storage types
//!
//! This module handles all in-memory store types
//! as well asynchronus persistent storage.
//!
//! By default, the module exports the [`MemStore`] type which can be used as in-memory storage
//! of key-value pairs.
//!
//! # Examples
//!
//! ```
//! use rubin::store::MemStore;
//!
//! let mut ms = MemStore::new();
//!
//! // Add a value to the store
//! ms.insert_string("key", "value");
//!
//! // Retrieve a value from the store
//! let result = ms.get_string("key").unwrap();
//! assert_eq!(&result, "value");
//! ```

pub mod persistence;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::io;

/// In-memory store of values
///
/// Used to store key-value pairs of strings with more features being added
/// as development continues.
#[derive(Default, Serialize, Deserialize)]
pub struct MemStore {
    /// Key-value store of string values
    pub strings: HashMap<String, String>,
}

impl MemStore {
    /// Constructs a default store of values
    ///
    /// # Exmaples
    ///
    /// ```
    /// use rubin::store::MemStore;
    ///
    /// let mut ms = MemStore::new();
    /// assert_eq!(ms.strings.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a string into the string store with a given key
    ///
    /// # Arguments
    ///
    /// * `key`: Key to store the value under
    /// * `value`: Value to store
    ///
    /// # Examples
    ///
    /// ```
    /// use rubin::store::MemStore;
    ///
    /// let mut ms = MemStore::new();
    /// let inserted_value = ms.insert_string("user:1000", "value").unwrap();
    /// assert_eq!(&inserted_value, "value");
    /// ```
    pub fn insert_string(&mut self, key: &str, value: &str) -> io::Result<String> {
        let _ = self.strings.insert(key.to_string(), value.to_string());

        Ok(value.to_string())
    }

    /// Retrieve a value from the string store
    ///
    /// # Arguments
    ///
    /// * `key`: Key of the value to retrieve
    ///
    /// # Examples
    ///
    /// ```
    /// use rubin::store::MemStore;
    ///
    /// let mut ms = MemStore::new();
    /// ms.insert_string("key", "value");
    ///
    /// let result = ms.get_string("key").unwrap();
    /// assert_eq!(&result, "value");
    /// ```
    pub fn get_string(&self, key: &str) -> io::Result<String> {
        if let Some(value) = self.strings.get(key) {
            return Ok(value.clone());
        }

        Ok("".to_string())
    }

    /// Remove a value from the string store
    ///
    /// # Arguments
    ///
    /// * `key`: Key to remove
    ///
    /// # Returns
    ///
    /// * value: Value that was removed from the store, empty string if not present
    ///
    /// # Example
    ///
    /// ```rust
    /// use rubin::store::MemStore;
    ///
    /// let mut ms = MemStore::new();
    ///
    /// ms.insert_string("user:1000", "value");
    ///
    /// let value = ms.remove_string("user:1000").unwrap();
    ///
    /// assert_eq!(&value, "value");
    /// ```
    pub fn remove_string(&mut self, key: &str) -> io::Result<String> {
        if let Some(value) = self.strings.remove(key) {
            return Ok(value);
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
