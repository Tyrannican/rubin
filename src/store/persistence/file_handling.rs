use std::io::Result;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::store::MemStore;

const STORAGE_FILE: &str = "rubinstore.json";

pub async fn create_directory<P: AsRef<Path>>(location: P) -> Result<PathBuf> {
    fs::create_dir_all(&location).await?;

    Ok(location.as_ref().to_path_buf())
}

pub async fn load_store(path: &PathBuf) -> Result<String> {
    let fp = path.join(STORAGE_FILE);
    let mut file = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(fp)
        .await?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    if contents.is_empty() {
        file.write_all(b"").await?;
    }

    Ok(contents)
}

pub async fn write_store(path: &PathBuf, store: &MemStore) -> Result<()> {
    let path = path.join(STORAGE_FILE);
    let raw = serde_json::to_string_pretty(&store)?;
    let mut file = fs::File::create(&path).await?;
    file.write_all(&raw.as_bytes()).await?;

    Ok(())
}
