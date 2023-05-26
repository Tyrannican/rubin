use std::io::Write;

const DEBUG: bool = true;
const PERSISTENT_STORE: &str = "./teststore";

async fn cleanup() -> std::io::Result<()> {
    if DEBUG {
        let mut response = String::new();
        print!("Remove? ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut response)?;

        if response.trim().to_lowercase() == "y" {
            tokio::fs::remove_dir_all(PERSISTENT_STORE).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    cleanup().await?;
    Ok(())
}
