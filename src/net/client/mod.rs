//! Client protocol for interacting with the Server
//!
//! The client connects to a running server and can make network requests to
//! retrieve items from the store.
//!
//! # Usage
//!
//! ```no_run
//! use rubin::net::client::RubinClient;
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let client = RubinClient::new("127.0.0.1", 9876);
//!
//!     let result = client.insert_string("user:1000", "value").await?;
//!
//!     assert_eq!(&result, "OK");
//!
//!     let value = client.get_string("user:1000").await?;
//!
//!     assert_eq!(&result, "value");
//!
//!     Ok(())
//! }
//! ```

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::net::parser::{create_request, parse_response, Operation};

use std::io::Result;

/// Client protocol for interacting with the Rubin Server
pub struct RubinClient {
    /// Address of the server
    pub address: String,
}

impl RubinClient {
    /// Creates a new client, storing the address
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rubin::net::client::RubinClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> std::io::Result<()> {
    ///     let client = RubinClient::new("127.0.0.1", 9876);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(addr: &str, port: usize) -> Self {
        let address = format!("{}:{}", addr, port);
        Self { address }
    }

    /// Sends a request to the server to insert a key-value pair into the string store
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rubin::net::client::RubinClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> std::io::Result<()> {
    ///     let client = RubinClient::new("127.0.0.1", 9876);
    ///     client.insert_string("username", "rubinuser").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn insert_string(&self, key: &str, value: &str) -> Result<String> {
        let msg = create_request(
            Operation::StringSet,
            vec![key.to_string(), value.to_string()],
        );
        self.request(&msg).await
    }

    /// Sends a request to the server to retrieve a value from the string store
    /// with the given key
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rubin::net::client::RubinClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> std::io::Result<()> {
    ///     let client = RubinClient::new("127.0.0.1", 9876);
    ///     let result = client.get_string("username").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_string(&self, key: &str) -> Result<String> {
        let msg = create_request(Operation::StringGet, vec![key.to_string()]);
        self.request(&msg).await
    }

    /// Sends a request to server and parses the response
    async fn request(&self, msg: &str) -> Result<String> {
        let response = self.send(&msg).await?;
        let contents = parse_response(&response);

        Ok(contents)
    }

    /// Sends a request to the server, returning the raw response
    async fn send(&self, msg: &str) -> Result<String> {
        let mut client = TcpStream::connect(&self.address).await?;
        client.write_all(msg.as_bytes()).await?;

        let mut buffer = [0; 4096];
        let n_bytes = client.read(&mut buffer).await?;
        if n_bytes == 0 {
            return Ok(String::from(""));
        }

        let response = String::from_utf8_lossy(&mut buffer[..n_bytes]);

        Ok(response.to_string())
    }
}
