//! An asynchronus in-memory store with on-disk persistence
//!
//! Functions as a wrapper around the [`MemStore`] struct with the option to write
//! to disk when needed in JSON format.
//!
//! A [`PersistentStore`] can be created in three ways:
//!
//! * Created from scratch with no previous state
//! * Loaded from disk using a previous store
//! * Created by consuming an already existing [`MemStore`]
//!
//! ## Creating a fresh Persistent Store
//!
//! Creating a fresh [`PersistentStore`] will create a storage directory (supplied by the user)
//! which will be used to store the contents of the inner [`MemStore`] in JSON format.
//!
//! This file is a hard-coded value of `rubinstore.json` although this may change in the future.
//!
//! ```no_run
//! use rubin::store::persistence::PersistentStore;
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let ps = PersistentStore::new("some/storage/location").await?;
//!     Ok(())
//! }
//!
//! ```
//!
//! ## Loading an existing store
//!
//! An already existing store file can be loaded to create a [`PersistentStore`]
//!
//! This will deserialize the contents into the inner [`MemStore`].
//!
//! ```no_run
//! use rubin::store::persistence::PersistentStore;
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let ps = PersistentStore::from_existing("some/existing/location").await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Consuming a MemStore
//!
//! A [`PersistentStore`] can be created by consuming the contents of an existing [`MemStore`]
//!
//! This will consume the [`MemStore`] and build a [`PersistentStore`] from the contents.
//!
//! ```no_run
//! use rubin::store::{MemStore, persistence::PersistentStore};
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let mut ms = MemStore::new();
//!     ms.insert_string("user:1000", "value");
//!
//!     let ps = PersistentStore::from_store("some/storage/location", ms).await?;
//!
//!     Ok(())
//! }
//! ```
pub(crate) mod file_handling;

use crate::store::persistence::file_handling::*;
use crate::store::MemStore;

use std::io;
use std::path::{Path, PathBuf};

/// In-memory key-value store with persistence
///
/// A wrapper around the [`MemStore`] with the option for on-disk persistence
/// in JSON format
pub struct PersistentStore {
    /// Directory which holds the store
    pub path: PathBuf,

    /// In-memory store
    pub store: MemStore,

    /// Whether to write to disk after each update or not
    pub write_on_update: bool,
}

impl PersistentStore {
    /// Create a fresh PersistentStore
    ///
    /// Will create the directory only, the store file is not created until after
    /// the first write operation.
    ///
    /// By default, writing on update is disabled but can be enabled using the
    /// [`Self::set_write_on_update()`]
    ///
    /// ```no_run
    /// use rubin::store::persistence::PersistentStore;
    ///
    /// #[tokio::main]
    /// async fn main() -> std::io::Result<()> {
    ///     let ps = PersistentStore::new("some/storage/location").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new<P: AsRef<Path>>(storage_loc: P) -> io::Result<Self> {
        let path = create_directory(storage_loc).await?;

        Ok(Self {
            path,
            store: MemStore::new(),
            write_on_update: false,
        })
    }

    /// Create a Persistent Store from an already existing store file.
    ///
    /// Will look in the given directory for a `rubinstore.json` file and load it from disk.
    ///
    /// This will deserialize the JSON into the inner [`MemStore`] type.
    ///
    /// ```no_run
    /// use rubin::store::persistence::PersistentStore;
    ///
    /// #[tokio::main]
    /// async fn main() -> std::io::Result<()> {
    ///     let ps = PersistentStore::from_existing("already/existing/store/directory").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn from_existing<P: AsRef<Path>>(storage_loc: P) -> io::Result<Self> {
        let mut store = Self::new(storage_loc).await?;
        store.load().await.expect("unable to load store");
        Ok(store)
    }

    /// Create a Persistent Store by consuming an existing [`MemStore`]
    ///
    /// This will perform the same operations as [`Self::new()`] but will consume a
    /// [`MemStore`] and its contents instead of creating a new one.
    ///
    /// ```no_run
    /// use rubin::store::{MemStore, persistence::PersistentStore};
    ///
    /// #[tokio::main]
    /// async fn main() -> std::io::Result<()> {
    ///     let mut ms = MemStore::new();
    ///     ms.insert_string("user:1000", "value")?;
    ///
    ///     let ps = PersistentStore::from_store("some/storage/location", ms).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn from_store<P: AsRef<Path>>(
        storage_loc: P,
        memstore: MemStore,
    ) -> io::Result<Self> {
        let path = create_directory(storage_loc).await?;

        Ok(Self {
            path,
            store: memstore,
            write_on_update: false,
        })
    }

    /// Insert a key-value pair into the string store
    ///
    /// Will only write to disk if `write_on_update` is set, otherwise it will act
    /// as a [`MemStore::insert_string()`]
    ///
    /// You can set to write on each update by using [`Self::set_write_on_update()`]
    ///
    /// ```no_run
    /// use rubin::store::persistence::PersistentStore;
    ///
    /// #[tokio::main]
    /// async fn main() -> std::io::Result<()> {
    ///     let mut ps = PersistentStore::new("./storage").await?;
    ///     ps.insert_string("user:1000", "value").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn insert_string(&mut self, key: &str, value: &str) -> io::Result<String> {
        let result = self.store.insert_string(key, value);

        if self.write_on_update {
            self.write().await?;
        }

        result
    }

    /// Retrieve a value from the string store denoted by the given key
    ///
    /// If no value is present, it will return an empty string
    ///
    /// ```no_run
    /// use rubin::store::persistence::PersistentStore;
    ///
    /// #[tokio::main]
    /// async fn main() -> std::io::Result<()> {
    ///     let mut ps = PersistentStore::new("./storage").await?;
    ///     ps.insert_string("user:1000", "value").await?;
    ///
    ///     // ...
    ///
    ///     let result = ps.get_string("user:1000")?;
    ///     assert_eq!(&result, "value");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_string(&self, key: &str) -> io::Result<String> {
        self.store.get_string(key)
    }

    /// Remove a value from the string store denoted by its key
    ///
    /// If no key is present, will return an empty string
    ///
    /// ```rust,no_run
    /// use rubin::store::persistence::PersistentStore;
    ///
    /// #[tokio::main]
    /// async fn main() -> std::io::Result<()> {
    ///     let mut ps = PersistentStore::new("./storage").await?;
    ///
    ///     ps.insert_string("user:1000", "value").await?;
    ///
    ///     let value = ps.remove_string("user:1000").await?;
    ///
    ///     assert_eq!(&value, "value");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn remove_string(&mut self, key: &str) -> io::Result<String> {
        let result = self.store.remove_string(key)?;

        if self.write_on_update {
            self.write().await?;
        }

        Ok(result)
    }

    /// Clears all strings from the string store
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rubin::store::persistence::PersistentStore;
    ///
    /// #[tokio::main]
    /// async fn main() -> std::io::Result<()> {
    ///     let mut ps = PersistentStore::new("./storage").await?;
    ///
    ///     for i in 0..100 {
    ///         let key = format!("key-{}", i);
    ///         ps.insert_string(&key, "value").await?;
    ///     }
    ///
    ///     ps.clear_strings().await?;
    ///
    ///     assert_eq!(ps.store.strings.len(), 0);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn clear_strings(&mut self) -> io::Result<()> {
        self.store.clear_strings()?;

        if self.write_on_update {
            self.write().await?;
        }

        Ok(())
    }

    /// Sets the store to perform a write after each update
    ///
    /// This should be set for cases where updates are infrequent as frequent writes
    /// on update can lead to a performance decrease.
    ///
    /// ```no_run
    /// use rubin::store::persistence::PersistentStore;
    ///
    /// #[tokio::main]
    /// async fn main() -> std::io::Result<()> {
    ///     let mut ps = PersistentStore::new("./storage").await?;
    ///     ps.set_write_on_update(true);
    ///
    ///     // The store will now write to disk on each update
    ///     ps.insert_string("user:1000", "value").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn set_write_on_update(&mut self, set: bool) {
        self.write_on_update = set;
    }

    /// Loads the store file from disk
    ///
    /// Parses the contents of the `rubinstore.json` file and deserializes it into
    /// a [`MemStore`]
    async fn load(&mut self) -> io::Result<()> {
        let contents = load_store(&self.path).await?;
        if contents.is_empty() {
            return Ok(());
        }

        let vault: MemStore = serde_json::from_str(&contents)?;

        self.store.strings = vault.strings;

        Ok(())
    }

    /// Writes the contents of the store out to disk
    ///
    /// This can be used to manually write the contents of the store out to disk
    /// when `set_write_on_update` is disabled.o
    ///
    /// This best suited for frequent updates when snapshotting each time is expensive.
    ///
    /// ```no_run
    /// use rubin::store::persistence::PersistentStore;
    ///
    /// #[tokio::main]
    /// async fn main() -> std::io::Result<()> {
    ///     let mut ps = PersistentStore::new("./storage").await?;
    ///
    ///     // No writing to disk
    ///     for i in 0..10_000 {
    ///         let key = format!("user:{}", i);
    ///         ps.insert_string(&key, "value").await?;
    ///     }
    ///
    ///     // Manually write to disk
    ///     ps.write().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn write(&self) -> io::Result<()> {
        write_store(&self.path, &self.store).await?;

        Ok(())
    }
}

#[cfg(test)]
mod persistent_store {
    use super::*;

    use std::path::PathBuf;
    use tempdir::TempDir;

    fn create_test_directory() -> io::Result<PathBuf> {
        let td = TempDir::new("teststore")?;
        Ok(td.path().to_path_buf())
    }

    #[tokio::test]
    async fn empty_store() -> io::Result<()> {
        let td = create_test_directory()?;
        let ps = PersistentStore::new(&td).await?;

        assert_eq!(ps.store.strings.len(), 0);
        assert_eq!(ps.path, td);

        Ok(())
    }

    #[tokio::test]
    async fn write_out_store() -> io::Result<()> {
        let td = create_test_directory()?;
        let rubinstore = td.join("rubinstore.json");
        let ps = PersistentStore::new(&td).await?;

        ps.write().await?;
        assert!(rubinstore.exists());

        Ok(())
    }

    #[tokio::test]
    async fn setting_write_on_update() -> io::Result<()> {
        let td = create_test_directory()?;
        let rubinstore = td.join("rubinstore.json");

        let mut ps = PersistentStore::new(&td).await?;
        assert!(!ps.write_on_update);

        ps.insert_string("key1", "value1").await?;
        assert!(!rubinstore.exists());

        ps.set_write_on_update(true);
        ps.insert_string("key2", "value2").await?;

        assert!(rubinstore.exists());

        Ok(())
    }

    #[tokio::test]
    async fn add_and_write() -> io::Result<()> {
        let td = create_test_directory()?;
        let rubinstore = td.join("rubinstore.json");

        let mut ps = PersistentStore::new(&td).await?;
        let result = ps.insert_string("key1", "value1").await?;

        assert_eq!(result, "value1");
        assert_eq!(ps.store.strings.len(), 1);

        ps.write().await?;
        assert!(rubinstore.exists());

        Ok(())
    }

    #[tokio::test]
    async fn add_a_load_of_strings() -> io::Result<()> {
        let td = create_test_directory()?;
        let rubinstore = td.join("rubinstore.json");
        let mut ps = PersistentStore::new(&td).await?;

        for i in 0..100_000 {
            let key = format!("key-{}", i);
            let value = format!("value-{}", i);
            let result = ps.insert_string(&key, &value).await?;
            assert_eq!(result, value);
        }

        ps.write().await?;
        assert!(rubinstore.exists());

        Ok(())
    }

    #[tokio::test]
    async fn load_existing_store() -> io::Result<()> {
        let td = create_test_directory()?;
        let mut ps = PersistentStore::new(&td).await?;
        ps.set_write_on_update(true);
        ps.insert_string("key1", "value1").await?;

        drop(ps);

        let ps = PersistentStore::from_existing(td).await?;
        assert_eq!(ps.store.strings.len(), 1);

        let result = ps.get_string("key1")?;
        assert_eq!(result, "value1");

        Ok(())
    }

    #[tokio::test]
    async fn load_from_memstore() -> io::Result<()> {
        let td = create_test_directory()?;
        let rubinstore = td.join("rubinstore.json");
        let mut ms = MemStore::new();

        for i in 0..10 {
            let key = format!("key-{}", i);
            let value = format!("value-{}", i);
            let _ = ms.insert_string(&key, &value);
        }

        let mut ps = PersistentStore::from_store(&td, ms).await?;
        ps.set_write_on_update(true);
        assert_eq!(ps.store.strings.len(), 10);

        let _ = ps.insert_string("key-11", "value-11").await?;
        assert_eq!(ps.store.strings.len(), 11);

        assert!(rubinstore.exists());

        Ok(())
    }
}
