use std::io::Result;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::store::Vault;

pub async fn create_directory(location: &str) -> Result<PathBuf> {
    let path = PathBuf::from(location);
    fs::create_dir_all(&path).await?;

    Ok(path)
}

pub async fn load_store(path: PathBuf) -> Result<String> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)
        .await?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    if contents.is_empty() {
        file.write_all(b"{}\n").await?;
    }

    Ok(contents)
}

pub async fn write_store(path: PathBuf, store: &Vault) -> Result<()> {
    let raw = serde_json::to_string_pretty(&store)?;
    let mut file = fs::File::create(&path).await?;
    file.write_all(&raw.as_bytes()).await?;

    Ok(())
}
