use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

use crate::persistence::{create_directory, load_store};

use serde::{Deserialize, Serialize};

const STORAGE_FILE: &str = "rubinstore.json";

#[derive(Debug, Clone, PartialEq)]
enum StorageType {
    Memory,
    Persistence,
}

impl Default for StorageType {
    fn default() -> Self {
        Self::Memory
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Vault {
    #[serde(skip)]
    path: PathBuf,

    #[serde(skip)]
    store_type: StorageType,

    pub strings: HashMap<String, String>,
}

impl Vault {
    pub fn memstore() -> Self {
        Self::default()
    }

    pub async fn with_persistence(storage_loc: &str) -> Self {
        let path = create_directory(storage_loc)
            .await
            .expect(&format!("unable to create directory: {}", storage_loc));

        let mut vault = Self {
            path,
            store_type: StorageType::Persistence,
            strings: HashMap::default(),
        };

        vault.load().await.expect("unable to load/create store");

        vault
    }

    pub fn insert_string(&mut self, key: &str, value: &str) -> io::Result<String> {
        let _ = self.strings.insert(key.to_string(), value.to_string());

        if self.store_type == StorageType::Persistence {}

        Ok(value.to_string())
    }

    pub fn get_string(&self, key: &str) -> io::Result<String> {
        dbg!(format!("Retrieving value for {}", key));
        if let Some(value) = self.strings.get(key) {
            return Ok(value.clone());
        }

        Ok("".to_string())
    }

    async fn load(&mut self) -> io::Result<()> {
        let store_file = self.path.join(STORAGE_FILE);
        let contents = load_store(store_file).await?;
        let vault: Vault = serde_json::from_str(&contents)?;

        self.strings = vault.strings;

        Ok(())
    }
}
