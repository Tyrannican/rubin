pub(crate) mod file_handling;

use crate::store::persistence::file_handling::*;
use crate::store::MemStore;
use std::io;

pub struct PersistentStore {
    pub store: MemStore,
}

impl PersistentStore {
    pub async fn new(storage_loc: &str) -> Self {
        let mut store = MemStore::new();
        let path = create_directory(storage_loc)
            .await
            .expect(&format!("unable to create directory: {}", storage_loc));

        store.path = path;

        Self { store }
    }

    async fn load(&mut self) -> io::Result<()> {
        let contents = load_store(&self.store.path).await?;
        if contents.is_empty() || contents == "{}\n" {
            return Ok(());
        }

        let vault: MemStore = serde_json::from_str(&contents)?;

        self.store.strings = vault.strings;

        Ok(())
    }

    async fn write(&self) -> io::Result<()> {
        write_store(&self.store).await?;

        Ok(())
    }
}
