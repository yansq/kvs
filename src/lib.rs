#![deny(missing_docs)]
//! A memory key-value store.
mod error;
use error::KvsError;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter};
use std::{path::PathBuf, result};

/// The result type of this crate.
pub type Result<T> = result::Result<T, KvsError>;

/// The store unit in file
#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    key: String,
    value: String,
}

/// A key-value store.
pub struct KvStore {
    file: File,
    reader: BufReader<File>,
    writer: BufWriter<File>,
    index: HashMap<String, u64>,
}

impl KvStore {
    /// Set a key-value pair, if the key already exists, it will be overwritten.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let log = Log { key, value };
        let log_json = serde_json::to_string(&log)?;

        self.writer.write_all(log_json.as_bytes())?;
        //TODO: add break line
        let offset = self.writer.stream_position()?;
        println!("offset:{}", offset);
        Ok(())
    }

    /// Get the value by a key, if the key does not exists, will return None.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        unimplemented!("unimplemented");
        Err(KvsError::Unknown)
    }

    /// Remove data by a key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        Ok(())
    }

    /// Open the KvStore at a given path.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let file = File::options().read(true).append(true).open(path.into())?;
        let reader_file = file.try_clone()?;
        let reader = BufReader::new(reader_file);
        let writer_file = file.try_clone()?;
        let writer = BufWriter::new(writer_file);

        let mut index = HashMap::new();
        let replay_file = file.try_clone()?;
        let replay_reader = BufReader::new(replay_file);
        Self::replay(replay_reader, &mut index);
        Ok(KvStore {
            file,
            reader,
            writer,
            index,
        })
    }

    // Replay from file, add (key, offser) paris to the memory
    fn replay(reader: BufReader<File>, index: &mut HashMap<String, u64>) {
        let mut stream = Deserializer::from_reader(reader).into_iter::<Log>();
        let mut offset = 0;
        while let Some(log) = stream.next() {
            if let Ok(log) = log {
                println!("log:{:?},offset:{:?}", log, offset);
                index.insert(log.key, offset);
                offset = stream.byte_offset() as u64;
            }
        }
    }
}
