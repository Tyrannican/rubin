mod client;

use client::handler;
use geymsla_lib::store::Vault;
use tokio::{net::TcpListener, sync::Mutex};

use std::sync::Arc;

const DEFAULT_GEYMSLA_PORT: usize = 9876;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let store = Arc::new(Mutex::new(Vault::empty()));
    let addr = format!("127.0.0.1:{}", DEFAULT_GEYMSLA_PORT);
    let listener = TcpListener::bind(&addr).await?;

    println!("Started Geymsla server");
    loop {
        let (client, _) = listener.accept().await?;
        let store = Arc::clone(&store);

        let client_addr = client.peer_addr()?;
        println!("Accepted new client: {}", client_addr);

        tokio::spawn(async move {
            handler(client, store).await;
        });
    }
}
