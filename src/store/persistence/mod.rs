pub(crate) mod file_handling;

use crate::store::persistence::file_handling::*;
use crate::store::MemStore;

use std::io;
use std::path::PathBuf;

pub struct PersistentStore {
    pub path: PathBuf,
    pub store: MemStore,
}

impl PersistentStore {
    pub async fn new(storage_loc: &str) -> Self {
        let path = create_directory(storage_loc)
            .await
            .expect(&format!("unable to create directory: {}", storage_loc));

        Self {
            path,
            store: MemStore::new(),
        }
    }

    pub async fn from_existing(storage_loc: &str) -> Self {
        let mut store = Self::new(storage_loc).await;
        store.load().await.expect("unable to load store");
        store
    }

    pub async fn from_store(storage_loc: &str, memstore: MemStore) -> Self {
        let path = create_directory(storage_loc)
            .await
            .expect(&format!("unable to create directory: {}", storage_loc));

        Self {
            path,
            store: memstore,
        }
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
