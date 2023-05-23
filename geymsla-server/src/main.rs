use geymsla_lib::{
    parser::{Message, OpCode},
    store::Vault,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use std::sync::Arc;

const DEFAULT_GEYMSLA_PORT: usize = 9876;

async fn send_response(client: &mut TcpStream, code: OpCode, msg: &str) {
    let response = format!("{}: {}\n", code.to_string(), msg);
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

    let message = match Message::new(msg) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("unable to create message: {}", e);
            return;
        }
    };

    let mut vault = store.lock().await;
    match message.op {
        OpCode::SSet => {
            let key = &message.contents[0];
            let value = &message.contents[1..].join(" ");

            if let Ok(_) = vault.insert_string(key, value) {
                send_response(&mut client, message.op, "OK").await;
            }
        }
        OpCode::SGet => {
            let key = &message.contents[0];
            if let Ok(value) = vault.get_string(key) {
                send_response(&mut client, message.op, &value).await;
            }
        }
        OpCode::Noop => {
            send_response(&mut client, message.op, "invalid command").await;
        }
    }

    dbg!(&vault.strings);
}

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
