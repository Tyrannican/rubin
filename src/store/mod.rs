//! In-memory and persistent storage types
//!
//! This module handles all in-memory store types
//! as well asynchronus persistent storage.
//!
//! By default, the module exports the [`MemStore`] type which can be used as in-memory storage
//! of key-value pairs.
//!
//! # Examples
//!
//! ```
//! use rubin::store::MemStore;
//!
//! let mut ms = MemStore::new();
//!
//! // Add a value to the store
//! ms.insert_string("key", "value");
//!
//! // Retrieve a value from the store
//! let result = ms.get_string("key").unwrap();
//! assert_eq!(&result, "value");
//! ```

pub mod inner_stores;
pub mod mem;
pub mod persistence;
