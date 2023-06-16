//! Client server protocol for running a store using network sockets
//!
//! This module contains implementations for the Server protocol which contains
//! the store and parses incoming requests from Clients to access items in the store.
//!
//! The Client requests information from the store and is returned over TCP sockets.
//!
//! It behaves similarly to Redis but is not feature complete as of yet.

pub mod client;
pub mod parser;
pub mod server;
