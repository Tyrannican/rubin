use std::sync::Arc;

use rubin_lib::{
    parser::{Message, OpCode},
    store::Vault,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};

async fn send_response(client: &mut TcpStream, code: OpCode, msg: &str) {
    let response = format!("{}: {}\n", code.to_string(), msg);
    client
        .write_all(response.as_bytes())
        .await
        .expect("unable to response to client");
}

async fn read_from_client(client: &mut TcpStream) -> String {
    let mut buffer = vec![0; 4096];
    let n_bytes = client
        .read(&mut buffer)
        .await
        .expect("unable to read from client");

    if n_bytes == 0 {
        return String::from("");
    }

    let msg = String::from_utf8_lossy(&buffer[..n_bytes]);
    msg.trim_end().to_string()
}

pub async fn handler(mut client: TcpStream, store: Arc<Mutex<Vault>>) {
    let msg = read_from_client(&mut client).await;

    let message = match Message::new(msg.as_str()) {
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
