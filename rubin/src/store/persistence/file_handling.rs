//! File-handling operations used by the [`super::PersistentStore`]
//!
//! Just a collection of File I/O helpers, nothing more, nothing less

use std::io::{Result, Write};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::store::mem::MemStore;

/// Creates a directory at the given location
pub async fn create_directory<P: AsRef<Path>>(location: P) -> Result<PathBuf> {
    fs::create_dir_all(&location).await?;

    Ok(location.as_ref().to_path_buf())
}

/// Loads a store file from disk.
///
/// Will read the contents of the file and return it as a String
/// If nothing is in the file, it will write an empty string to the file
pub async fn load_store(path: &Path) -> Result<String> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)
        .await?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    if contents.is_empty() {
        file.write_all(b"").await?;
    }

    Ok(contents)
}

/// Serializes a [`MemStore`] and saves it out to disk
pub async fn write_store(path: &Path, store: &MemStore) -> Result<()> {
    let raw = serde_json::to_string_pretty(&store)?;
    let mut file = fs::File::create(&path).await?;
    file.write_all(raw.as_bytes()).await?;

    Ok(())
}

/// Serializes a [`MemStore`] and saves it to disk without async functionality
pub fn write_store_sync(path: impl AsRef<Path>, store: &MemStore) -> Result<()> {
    let raw = serde_json::to_string_pretty(&store)?;
    let mut file = std::fs::File::create(&path)?;
    file.write_all(raw.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod fh_tests {
    use super::*;
    use std::io;
    use std::path::PathBuf;
    use tempdir::TempDir;

    const STORAGE_FILE: &str = "rubinstore.json";

    fn create_test_directory() -> io::Result<PathBuf> {
        let td = TempDir::new("teststore")?;
        Ok(td.path().to_path_buf())
    }

    #[tokio::test]
    async fn creating_a_directory() -> io::Result<()> {
        let td = create_test_directory()?;
        let expected = td.join("should_be_a_new_dir");

        let result = create_directory(&expected).await?;

        assert_eq!(expected, result);
        assert!(result.exists());
        Ok(())
    }

    #[tokio::test]
    async fn loading_an_empty_store() -> io::Result<()> {
        let td = create_test_directory()?;
        let rubinstore = td.join(STORAGE_FILE);
        create_directory(&td).await?;

        let result = load_store(&rubinstore).await?;
        assert!(result.len() == 0);
        assert!(rubinstore.exists());
        Ok(())
    }

    #[tokio::test]
    async fn loading_an_existing_store() -> io::Result<()> {
        let td = create_test_directory()?;
        let rubinstore = td.join(STORAGE_FILE);
        create_directory(&td).await?;

        let mut f = tokio::fs::File::create(&rubinstore).await?;
        f.write_all(b"some_content").await?;

        let result = load_store(&rubinstore).await?;
        assert!(result.len() != 0);
        assert_eq!(result, "some_content");

        Ok(())
    }

    #[tokio::test]
    async fn write_a_store_out() -> io::Result<()> {
        let td = create_test_directory()?;
        let rubinstore = td.join("rubinstore.json");
        create_directory(&td).await?;

        let mut ms = MemStore::new();
        ms.insert_string("key1", "value1")?;

        write_store(&rubinstore, &ms).await?;

        assert!(rubinstore.exists());

        Ok(())
    }

    #[tokio::test]
    async fn write_a_store_out_and_compare() -> io::Result<()> {
        let td = create_test_directory()?;
        let rubinstore = td.join("rubinstore.json");
        create_directory(&td).await?;

        let mut ms = MemStore::new();
        ms.insert_string("key1", "value1")?;

        write_store(&rubinstore, &ms).await?;

        assert!(rubinstore.exists());

        let contents = load_store(&rubinstore).await?;
        let other: MemStore = serde_json::from_str(&contents)?;
        assert!(ms.strings.inner == other.strings.inner);

        Ok(())
    }

    #[tokio::test]
    async fn write_a_store_out_sync() -> io::Result<()> {
        let td = create_test_directory()?;
        let rubinstore = td.join("rubinstore.json");
        create_directory(&td).await?;

        let mut ms = MemStore::new();
        ms.insert_string("key1", "value1")?;

        write_store_sync(&rubinstore, &ms)?;

        assert!(rubinstore.exists());

        Ok(())
    }
}
