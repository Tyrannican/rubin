//! An asynchronus in-memory store with on-disk persistence
//!
//! Functions as a wrapper around the [MemStore] struct with the option to write
//! to disk when needed.
//!
//! TODO: Add examples with each type of creation method
//!
//! ## Examples
//!
//! ```no_run
//! use rubin::store::persistence::PersistentStore;
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let mut ps = PersistentStore::new("some/storage/location").await?;
//!     Ok(())
//! }
//!
//! ```
pub(crate) mod file_handling;

use crate::store::persistence::file_handling::*;
use crate::store::MemStore;

use std::io;
use std::path::{Path, PathBuf};

pub struct PersistentStore {
    pub path: PathBuf,
    pub store: MemStore,
    pub write_on_update: bool,
}

impl PersistentStore {
    pub async fn new<P: AsRef<Path>>(storage_loc: P) -> io::Result<Self> {
        let path = create_directory(storage_loc).await?;

        Ok(Self {
            path,
            store: MemStore::new(),
            write_on_update: false,
        })
    }

    pub async fn from_existing<P: AsRef<Path>>(storage_loc: P) -> io::Result<Self> {
        let mut store = Self::new(storage_loc).await?;
        store.load().await.expect("unable to load store");
        Ok(store)
    }

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

    pub async fn insert_string(&mut self, key: &str, value: &str) -> io::Result<String> {
        let result = self.store.insert_string(key, value);

        if self.write_on_update {
            self.write().await?;
        }

        result
    }

    pub fn get_string(&self, key: &str) -> io::Result<String> {
        self.store.get_string(key)
    }

    pub fn set_write_on_update(&mut self, set: bool) {
        self.write_on_update = set;
    }

    async fn load(&mut self) -> io::Result<()> {
        let contents = load_store(&self.path).await?;
        if contents.is_empty() {
            return Ok(());
        }

        let vault: MemStore = serde_json::from_str(&contents)?;

        self.store.strings = vault.strings;

        Ok(())
    }

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
