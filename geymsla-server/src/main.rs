use geymsla_lib::store::Vault;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use std::sync::Arc;

const DEFAULT_GEYMSLA_PORT: usize = 9876;

async fn send_response(client: &mut TcpStream, code: &str, msg: &str) {
    let response = format!("{}: {}\n", code, msg);
    client
        .write_all(response.as_bytes())
        .await
        .expect("unable to response to client");
}

async fn handler(mut client: TcpStream, store: Arc<Mutex<Vault>>) {
    let mut buffer = vec![0; 4096];
    let n_bytes = client
        .read(&mut buffer)
        .await
        .expect("unable to read from client");

    if n_bytes == 0 {
        return;
    }

    let msg = String::from_utf8_lossy(&buffer[..n_bytes]);
    let msg = msg.trim_end();

    // TODO: Split this into somewhere else
    let msg_tokens = msg.split(' ').collect::<Vec<&str>>();
    let op_code = msg_tokens[0];
    let rest = &msg_tokens[1..];
    let mut vault = store.lock().await;
    match op_code {
        "SSET" => {
            let key = rest[0];
            let value = rest[1];

            if let Ok(_) = vault.insert_string(key, value) {
                send_response(&mut client, op_code, "OK").await;
            }
        }
        "SGET" => {
            let key = rest[0];
            if let Ok(value) = vault.get_string(key) {
                send_response(&mut client, op_code, &value).await;
            }
        }
        _ => {
            send_response(&mut client, op_code, "NOOP").await;
        }
    }

    println!("Current values: {:#?}", vault.strings);
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let store = Arc::new(Mutex::new(Vault::empty()));
    let addr = format!("0.0.0.0:{}", DEFAULT_GEYMSLA_PORT);
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
