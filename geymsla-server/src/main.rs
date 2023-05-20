use std::collections::HashMap;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::Mutex,
};

use std::sync::Arc;

const DEFAULT_GEYMSLA_PORT: usize = 9876;

pub struct Geymsla {
    pub strings: HashMap<String, String>,
}

impl Geymsla {
    pub fn empty() -> Self {
        Self {
            strings: HashMap::default(),
        }
    }

    pub fn insert_string(&mut self, string: &str) -> std::io::Result<String> {
        println!("Inserting string: {string}");

        Ok(string.to_string())
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let geymsla = Arc::new(Mutex::new(Geymsla::empty()));
    let addr = format!("0.0.0.0:{}", DEFAULT_GEYMSLA_PORT);
    let listener = TcpListener::bind(&addr).await?;

    println!("Started Geymsla server");
    loop {
        let (mut client, _) = listener.accept().await?;
        let store = Arc::clone(&geymsla);

        println!("Accepted new client: {}", &client.peer_addr()?);

        tokio::spawn(async move {
            let mut buffer = vec![0; 4096];
            let n_bytes = client
                .read(&mut buffer)
                .await
                .expect("unable to read from client");

            if n_bytes == 0 {
                return;
            }

            let msg = String::from_utf8(buffer).expect("unable to convert buffer to message");
            let mut geymsla = store.lock().await;
            let _ = geymsla.insert_string(msg.as_ref());
            client
                .write_all(b"success")
                .await
                .expect("unable to response to client");
        });
    }
}
