//! In-memory store with the option for on-disk persistence.
//!
//! This library is designed as a lightweight Redis-like clone for simple in-memory storage of
//! key-value pairs.
//!
//! This project is a very rough WIP with more features planned at a future date.
//!
//! # Usage as a library
//!
//! The primary use case for this library is to be used within project code.
//!
//! It can act as an in-memory key-value store to store and retrieve values from when required.
//! The store can also be used for persistence by serializing the data within and storing it to
//! disk.
//!
//! ## Usage as an In-Memory store
//!
//! This is the default use-case in which the [`store::MemStore`] is used as the key-value store.
//! Values can be added and retrieved when needed and all data is lost upon the Store being
//! dropped.
//!
//! ```rust
//! use rubin::store::mem::MemStore;
//!
//! let mut ms = MemStore::new();
//!
//! // Store a value for later use
//! ms.insert_string("key-1", "value1");
//!
//! // ...
//!
//! let value = ms.get_string("key-1").unwrap();
//!
//! // Use the value when needed.
//! ```
//!
//! ## Usage as a persistent store
//!
//! The store can be used as a persistent store which acts in the same way as the in-memory store
//! but with the option to save the store contents out to disk in JSON format.
//!
//! This can be useful when fast data access is required but retaining data is required.
//!
//! The [`store::persistence::PersistentStore`] uses asynchronus file I/O using [`tokio::fs`] to
//! save / load data from disk.
//!
//! ```rust,no_run
//! use rubin::store::persistence::PersistentStore;
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     // Create a Persistent store which automatically creates a storage location from the given
//!     // path.
//!     let mut ps = PersistentStore::new("./storage_folder").await?;
//!
//!     // By default, the store does not write to disk automatically
//!     ps.insert_string("key-1", "value1").await?;
//!
//!     // Manually writing to disk
//!     // All data is saved to a file called `rubinstore.json` in the storage location
//!     ps.write().await?;
//!
//!     // A flag can be set to write automatically after each update
//!     ps.set_write_on_update(true);
//!
//!     // After the flag is set, the values are written to disk automatically
//!     ps.insert_string("key-2", "value2").await?;
//!
//!     // Retrieving a value from the Store
//!     // As this is using the in-memory store, no async operations area required.
//!     let value = ps.get_string("key-1");
//!
//!     Ok(())
//! }
//! ```
//!
//! # Usage as a client / server
//!
//! The library also offers the ability to operate the store as in-memory network storage using TCP
//! sockets facilitated by [`tokio::net`].
//!
//! A server can be started which will operate a [`store::MemStore`] which can be communicated with
//! via the [`net::client::RubinClient`] that can be used to store and retrieve values from the
//! server.
//!
//! ## Using the Server
//!
//! The library offers the ability to start a server which runs as a TCP socket waiting for
//! connections. As there are many ways to facilitate this, the implementation of how the server
//! operates is up to the end-user.
//!
//! Currently, the server only acts as an in-memory store with no option for persistence but this
//! is a planned feature upgrade in the future.
//!
//! ### Basic Example
//!
//! ```rust,no_run
//! use rubin::net::server::start;
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     // Start a server as a separate Tokio task
//!     tokio::task::spawn(start("127.0.0.1", 9876));
//!
//!     // Endless loop but the actual implementation is up to the user.
//!     loop {}
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Using the Client
//!
//! A client is provided which can be used to connect to an already running server which can be
//! used to insert and retrieve values into the server store.
//!
//! The connection is only made when an operation is requested and will not persist after the
//! operation has finished.
//!
//! ```rust,no_run
//! use rubin::net::client::RubinClient;
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     // Create a client which stores the address for a later point
//!     let client = RubinClient::new("127.0.0.1", 9876);
//!
//!     /// ...
//!     
//!     // Set a value in the server
//!     // This will connect to the server and send the request
//!     // After each operation, the connection is dropped
//!     client.insert_string("user:1000", "value").await?;
//!
//!     // ...
//!
//!     // Request a value from the server
//!     // The server will then response with the value if it is present
//!     let value = client.get_string("user:1000").await?;
//!
//!     Ok(())
//! }
//! ```
pub mod net;

pub mod errors;

pub mod store;
