//! In-memory data store for a collection of types.
//!
//! Stores items of different types in a key-value store accessible via a key.
//!
//! # Supported types:
//!
//! * `Strings`: Store string values
//!
//! # Future supported types:
//!
//! * `Lists`: Store for a `Vec<T>` of items with `T` being a generic type
//! * `HashMap`: Store a `HashMap` of values
//! * `Set`: Store a `Set` of values
//!
//! As development continues, more features will be added.
//!
//! # Example
//!
//! ```
//! use rubin::store::mem::MemStore;
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

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::io;

use crate::store::persistence::file_handling::write_store_sync;
use crate::store::InnerStore;

/// In-memory store of values
///
/// Used to store key-value pairs of strings with more features being added
/// as development continues.
#[derive(Default, Serialize, Deserialize)]
pub struct MemStore {
    /// Key-value store of `String` values
    pub strings: InnerStore<String>,
}

impl MemStore {
    /// Constructs a default store of values
    ///
    /// # Examples
    ///
    /// ```
    /// use rubin::store::mem::MemStore;
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
    /// use rubin::store::mem::MemStore;
    ///
    /// let mut ms = MemStore::new();
    /// ms.insert_string("user:1000", "value").unwrap();
    /// ```
    pub fn insert_string(&mut self, key: &str, value: &str) -> io::Result<()> {
        self.strings.insert(key, value.to_string())
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
    /// use rubin::store::mem::MemStore;
    ///
    /// let mut ms = MemStore::new();
    /// ms.insert_string("key", "value");
    ///
    /// let result = ms.get_string("key").unwrap();
    /// assert_eq!(&result, "value");
    /// ```
    pub fn get_string(&self, key: &str) -> io::Result<String> {
        self.strings.retrieve(key)
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
    /// use rubin::store::mem::MemStore;
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
        self.strings.remove(key)
    }

    /// Clears all entries out of the string store
    ///
    /// # Example
    ///
    /// ```rust
    /// use rubin::store::mem::MemStore;
    ///
    /// let mut ms = MemStore::new();
    ///
    /// for i in 0..100 {
    ///     let key = format!("key-{}", i);
    ///     ms.insert_string(&key, "value");
    /// }
    ///
    /// ms.clear_strings();
    ///
    /// assert_eq!(ms.strings.len(), 0);
    /// ```
    pub fn clear_strings(&mut self) -> io::Result<()> {
        self.strings.clear()
    }

    /// Gets a shared reference to the inner string store.
    ///
    /// Used for more complicated operations not offered by the APIo
    ///
    /// # Example
    ///
    /// ```rust
    /// use rubin::store::mem::MemStore;
    ///
    /// let mut ms = MemStore::new();
    ///
    /// // ...
    ///
    /// let strings = ms.get_string_store_ref();
    ///
    /// for (key, value) in strings.iter() {
    ///     println!("{} {}", key, value);
    /// }
    /// ```
    pub fn get_string_store_ref(&self) -> &HashMap<String, String> {
        self.strings.get_ref()
    }

    /// Writes the contents of the store out to disk.
    ///
    /// Used in scenarios where you want to dump the contents out to disk but do not
    /// want to run with persistence.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rubin::store::mem::MemStore;
    ///
    /// let mut ms = MemStore::new();
    ///
    /// // ...
    ///
    /// ms.dump_store("save/path/location.json");
    /// ```
    pub fn dump_store(&self, filepath: impl AsRef<std::path::Path>) -> io::Result<()> {
        write_store_sync(filepath, &self)
    }
}

#[cfg(test)]
mod memstore {
    use super::*;
    use std::path::PathBuf;
    use tempdir::TempDir;

    fn create_test_directory() -> io::Result<PathBuf> {
        let td = TempDir::new("teststore")?;
        Ok(td.path().to_path_buf())
    }

    #[test]
    fn empty() {
        let ms = MemStore::new();
        let strings = ms.strings.get_ref();
        assert_eq!(strings.len(), 0);
    }

    #[test]
    fn string_store_add_entries() -> io::Result<()> {
        let mut ms = MemStore::new();
        ms.insert_string("key1", "value1")?;

        assert_eq!(ms.strings.len(), 1);

        ms.insert_string("key2", "value2")?;

        assert_eq!(ms.strings.len(), 2);

        Ok(())
    }

    #[test]
    fn string_store_add_loads_of_entries() -> io::Result<()> {
        let mut ms = MemStore::new();

        for i in 0..100_000 {
            let key = format!("key-{}", i);
            let value = format!("value-{}", i);
            ms.insert_string(&key, &value)?;
        }

        assert!(ms.strings.len() == 100_000);

        Ok(())
    }

    #[test]
    fn string_store_get_entries() -> io::Result<()> {
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
    fn string_store_no_entries() -> io::Result<()> {
        let ms = MemStore::new();
        let result = ms.get_string("key1")?;
        assert_eq!(result, "");

        Ok(())
    }

    #[test]
    fn clear_string_store() -> io::Result<()> {
        let mut ms = MemStore::new();
        for i in 0..1000 {
            let key = format!("key-{}", i);
            ms.insert_string(&key, "value")?;
        }

        assert!(ms.strings.len() == 1000);
        ms.clear_strings()?;
        assert!(ms.strings.len() == 0);

        Ok(())
    }

    #[test]
    fn dump_store_to_disk() -> io::Result<()> {
        let td = create_test_directory()?;
        let rubinstore = td.join("rubinstore.json");
        std::fs::create_dir_all(td)?;

        let mut ms = MemStore::new();
        ms.insert_string("key1", "value1")?;

        ms.dump_store(&rubinstore)?;

        assert!(rubinstore.exists());

        Ok(())
    }
}
