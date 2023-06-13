pub mod string_store;

use std::io;

pub trait StoreOperations {
    type StoreType;

    fn insert(&mut self, key: &str, value: Self::StoreType) -> io::Result<Self::StoreType>;
    fn retrieve(&self, key: &str) -> io::Result<Self::StoreType>;
    fn remove(&mut self, key: &str) -> io::Result<Self::StoreType>;
    fn clear(&mut self);
}
