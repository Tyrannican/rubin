//! In-memory and persistent storage types
//!
//! This module handles all in-memory store types
//! as well asynchronus persistent storage.
//!
//! For examples, see [`mem`] and [`persistence`] module documentation.

pub mod mem;
pub mod persistence;

use std::collections::HashMap;
use std::io;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// Generic struct representing an inner store of a given type.
///
/// This is intended to be used by the [`mem::MemStore`] as an internal type and is not directly
/// used.
#[derive(Default, Serialize, Deserialize)]
pub struct InnerStore<T> {
    /// Storage for the generic type
    inner: HashMap<String, T>,
}

impl<T> InnerStore<T>
where
    T: Default + Clone + Serialize + DeserializeOwned,
{
    /// Get the total number of items in the store
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Checks if the store is currently empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Insert an item into the store
    pub fn insert(&mut self, key: &str, value: T) -> io::Result<()> {
        self.inner.insert(key.to_string(), value);
        Ok(())
    }

    /// Retrieve an item from the store denoted by the key
    ///
    /// * Returns:
    ///     * The value if it is present
    ///     * Default value for the type if not (i.e. empty)
    pub fn retrieve(&self, key: &str) -> io::Result<T> {
        if let Some(value) = self.inner.get(key) {
            return Ok(value.clone());
        }

        Ok(T::default())
    }

    /// Remove an item from the store and return the removed value
    pub fn remove(&mut self, key: &str) -> io::Result<T> {
        if let Some(value) = self.inner.remove(key) {
            return Ok(value);
        }

        Ok(T::default())
    }

    /// Clear all items in the store
    pub fn clear(&mut self) -> io::Result<()> {
        self.inner.clear();
        Ok(())
    }

    /// Gets an immutable reference to the inner store type
    pub fn get_ref(&self) -> &HashMap<String, T> {
        &self.inner
    }
}
