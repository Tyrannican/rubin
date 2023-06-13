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

pub mod mem;
pub mod persistence;

use std::collections::HashMap;
use std::io;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct InnerStore<T> {
    inner: HashMap<String, T>,
}

impl<T> InnerStore<T>
where
    T: Default + Clone + Serialize + DeserializeOwned,
{
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn insert(&mut self, key: &str, value: T) -> io::Result<()> {
        self.inner.insert(key.to_string(), value);
        Ok(())
    }

    pub fn retrieve(self, key: &str) -> io::Result<T> {
        if let Some(value) = self.inner.get(key) {
            return Ok(value.clone());
        }

        Ok(T::default())
    }

    pub fn remove(&mut self, key: &str) -> io::Result<T> {
        if let Some(value) = self.inner.remove(key) {
            return Ok(value);
        }

        Ok(T::default())
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.inner.clear();
        Ok(())
    }

    pub fn get_ref(&self) -> &HashMap<String, T> {
        &self.inner
    }
}
