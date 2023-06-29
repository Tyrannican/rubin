//! Server protocol for operating a store on a network
//!
//! Creates a [`MemStore`] which operates over a network, accepting requests from clients
//! to interact with the store.
//!
//! Can be run as an asynchronus task or as a background process, usage depends on end-user wants
//! and needs.

use std::sync::Arc;

use crate::{
    errors::MessageError,
    net::parser::{parse_request, Operation},
    store::mem::MemStore,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use tracing::{debug, error, info};
use tracing_subscriber::FmtSubscriber;

static INIT_TRACING: std::sync::Once = std::sync::Once::new();
/// Sets up a global logger
fn init_logger() {
    INIT_TRACING.call_once(|| {
        FmtSubscriber::builder().init();
    })
}

/// Sends a formatted response to the client prefixed with the [`Operation`] tag
async fn send_response(client: &mut TcpStream, code: Operation, msg: &str) {
    let response = format!("{}::{}\n", code, msg);
    client
        .write_all(response.as_bytes())
        .await
        .expect("unable to response to client");
}

/// Reads a message from a client and converts it to a string
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

/// Main handler for the server
///
/// Processes incoming requests from the client and performs the requested operation.
/// If the operation cannot be processed, an error is returned.
async fn handler(mut client: TcpStream, store: Arc<Mutex<MemStore>>) {
    let client_address = client
        .peer_addr()
        .expect("unable to get client address")
        .to_string();

    let msg = read_from_client(&mut client).await;
    info!("{} -> {}", client_address, msg);

    let message = match parse_request(&msg) {
        Ok(msg) => msg,
        Err(error) => {
            match error {
                MessageError::InvalidMessage(msg) | MessageError::InvalidFormat(msg) => {
                    error!("failed to parse message - {}", msg);
                    send_response(&mut client, Operation::Error, &msg).await
                }
            }
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
            info!("{} <- {}", client_address, "OK");
        }
        Operation::StringGet => {
            let key = &message.args[0];

            if let Ok(value) = vault.get_string(key) {
                send_response(&mut client, message.op, &value).await;
                info!("{} <- {}", client_address, &value);
            }
        }
        Operation::StringRemove => {
            let key = &message.args[0];

            if let Ok(value) = vault.remove_string(key) {
                send_response(&mut client, message.op, &value).await;
                info!("{} <- {}", client_address, &value);
            }
        }
        Operation::StringClear => {
            if vault.clear_strings().is_ok() {
                send_response(&mut client, message.op, "OK").await;
                info!("{} <- {}", client_address, "OK");
            }
        }
        Operation::Dump => {
            let filepath = &message.args[0];
            if let Ok(_) = vault.dump_store(filepath) {
                send_response(&mut client, message.op, "OK").await;
                info!("{} <- {}", client_address, "OK");
            }
        }
        _ => {
            send_response(&mut client, Operation::Noop, "nothing to do").await;
            info!("{} <- noop", client_address);
        }
    }
}

/// Starts the server to accept clients
///
/// This can be run as an independent task or as part of separate binary.
///
/// The usage really depends on what the end-user wants so this offers a simple
/// function to start the server.
///
/// An example could be that this function could be started as a Daemon using a
/// crate (e.g. [Daemonize](https://crates.io/crates/daemonize))
///
/// # Usage
///
/// ```no_run
/// use rubin::net::server::start;
///
/// #[tokio::main]
/// async fn main() -> std::io::Result<()> {
///     tokio::task::spawn(start("127.0.0.1", 9876));
///
///     // Rest of the workload
///
///     Ok(())
/// }
/// ```
pub async fn start(addr: &str, port: usize) -> std::io::Result<()> {
    init_logger();
    let store = Arc::new(Mutex::new(MemStore::new()));
    let addr = format!("{}:{}", addr, port);
    let listener = TcpListener::bind(&addr).await?;

    info!("Started Rubin server at {}", addr);
    loop {
        let (client, _) = listener.accept().await?;
        let store = Arc::clone(&store);

        let client_addr = client.peer_addr()?;
        debug!("Accepted new client: {}", client_addr);

        tokio::spawn(async move {
            handler(client, store).await;
        });
    }
}
