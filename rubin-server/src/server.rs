use std::sync::Arc;

use rubin_lib::{
    net::parser::{parse_request, Operation},
    store::MemStore,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};

async fn send_response(client: &mut TcpStream, code: Operation, msg: &str) {
    let response = format!("{}::{}\n", code.to_string(), msg);
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

pub async fn handler(mut client: TcpStream, store: Arc<Mutex<MemStore>>) {
    let msg = read_from_client(&mut client).await;

    let message = match parse_request(&msg) {
        Ok(msg) => msg,
        Err(_) => {
            send_response(&mut client, Operation::Error, "invalid message").await;
            return;
        }
    };

    let mut vault = store.lock().await;
    match message.op {
        Operation::StringSet => {
            let key = &message.args[0];
            let value = &message.args[1];

            let _ = vault.insert_string(key, value);
            send_response(&mut client, message.op, "OK").await;
        }
        Operation::StringGet => {
            let key = &message.args[0];

            if let Ok(value) = vault.get_string(key) {
                send_response(&mut client, message.op, &value).await;
            }
        }
        _ => {
            send_response(&mut client, Operation::Noop, "nothing to do").await;
        }
    }

    dbg!(&vault.strings);
}
