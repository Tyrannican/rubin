pub(crate) mod file_handling;

use crate::store::persistence::file_handling::*;
use crate::store::MemStore;

use std::io;
use std::path::{Path, PathBuf};

pub struct PersistentStore {
    pub path: PathBuf,
    pub store: MemStore,
}

impl PersistentStore {
    pub async fn new<P: AsRef<Path>>(storage_loc: P) -> io::Result<Self> {
        let path = create_directory(storage_loc).await?;

        Ok(Self {
            path,
            store: MemStore::new(),
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
        })
    }

    pub async fn insert_string(&mut self, key: &str, value: &str) -> io::Result<String> {
        let inserted = self.store.insert_string(key, value);
        self.write().await?;

        inserted
    }

    pub fn get_string(&self, key: &str) -> io::Result<String> {
        self.store.get_string(key)
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

    async fn write(&self) -> io::Result<()> {
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
    async fn add_and_write() -> io::Result<()> {
        let td = create_test_directory()?;
        let rubinstore = td.join("rubinstore.json");

        let mut ps = PersistentStore::new(&td).await?;
        let result = ps.insert_string("key1", "value1").await?;

        assert_eq!(result, "value1");
        assert_eq!(ps.store.strings.len(), 1);
        assert!(rubinstore.exists());

        Ok(())
    }
}
