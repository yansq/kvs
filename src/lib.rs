#![deny(missing_docs)]
//! A memory key-value store.

use std::{collections::HashMap, path::PathBuf};

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
    pub fn set(&mut self, key: String, value: String) -> Result<(), String> {
        self.data.insert(key, value);
        Ok(())
    }

    /// Get the value by a key, if the key does not exists, will return None.
    pub fn get(&mut self, key: String) -> Option<String> {
        if self.data.contains_key(&key) {
            return self.data.get(&key).cloned();
        } else {
            None
        }
    }

    /// Remove data by a key.
    pub fn remove(&mut self, key: String) {
        if self.data.contains_key(&key) {
            self.data.remove(&key);
        }
    }

    /// Open the KvStore at a given path.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore, String> {
        unimplemented!("unimplemented");
    }
}
