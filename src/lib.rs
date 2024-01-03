#![deny(missing_docs)]
//! A memory key-value store.
mod error;

use error::KvsError;
use std::{collections::HashMap, path::PathBuf, result};

/// The result type of this crate.
pub type Result<T> = result::Result<T, KvsError>;

/// A key-value store.
#[derive(Default)]
pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    /// Init an empty KvStore, which is a HashMap, to store data.
    pub fn new() -> KvStore {
        Default::default()
    }

    /// Set a key-value pair, if the key already exists, it will be overwritten.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.data.insert(key, value);
        unimplemented!("unimplemented");
    }

    /// Get the value by a key, if the key does not exists, will return None.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if self.data.contains_key(&key) {
            return Ok(self.data.get(&key).cloned());
        } else {
            Err(KvsError::Unknown)
        }
    }

    /// Remove data by a key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.data.contains_key(&key) {
            self.data.remove(&key);
        }
        Ok(())
    }

    /// Open the KvStore at a given path.
    pub fn open(_path: impl Into<PathBuf>) -> Result<KvStore> {
        Ok(KvStore::new())
    }
}
