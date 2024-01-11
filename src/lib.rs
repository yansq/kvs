#![deny(missing_docs)]
//! A memory key-value store.
mod error;

use clap::Subcommand;
use error::KvsError;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter};
use std::{path::PathBuf, result};
use walkdir::WalkDir;

/// The result type of this crate.
pub type Result<T> = result::Result<T, KvsError>;

/// The store unit in file
#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    key: String,
    value: String,
}

/// The store unit in file
#[derive(Debug, Subcommand, Deserialize, Serialize)]
pub enum Command {
    /// Set a key-value pair
    Set {
        /// key
        #[arg(value_name = "KEY")]
        key: String,
        /// value
        #[arg(value_name = "VALUE")]
        value: String,
    },
    /// Get the value by a key
    Get {
        /// key
        #[arg(value_name = "KEY")]
        key: String,
    },
    /// Remove a key
    Rm {
        /// key
        #[arg(value_name = "KEY")]
        key: String,
    },
}

/// A key-value store.
pub struct KvStore {
    path: PathBuf,
    reader: BufReader<File>,
    writer: BufWriter<File>,
    index: HashMap<String, u64>,
    offset: u64,
}

impl KvStore {
    /// Set a key-value pair, if the key already exists, it will be overwritten.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::Set {
            key: key.clone(),
            value,
        };
        let log_json = serde_json::to_string(&command)?;

        self.writer.write_all(log_json.as_bytes())?;
        self.index.insert(key, self.offset);
        self.offset = self.writer.stream_position()?;

        let dir_size = Self::dir_size(self.path.clone());
        if dir_size > 10240 && !self.path.parent().unwrap().join("store_new.txt").exists() {
            self.compact()?;
        }

        Ok(())
    }

    /// Get the value by a key, if the key does not exists, will return None.
    /// Search the index map, if not found, return None;
    /// otherwise use the offset in index map to read data from file and return.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(offset) = self.index.get(&key) {
            self.reader.seek(std::io::SeekFrom::Start(*offset))?;
            let mut stream =
                serde_json::Deserializer::from_reader(&mut self.reader).into_iter::<Command>();
            if let Some(log) = stream.next() {
                if let Command::Set { value, .. } = log? {
                    return Ok(Some(value));
                }
            }
        }
        println!("Key not found");
        Ok(None)
    }

    /// Remove data by a key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.index.get(&key).is_some() {
            self.index.remove(&key);
            let command = Command::Rm { key: key.clone() };
            let log = serde_json::to_string(&command)?;
            self.writer.write_all(log.as_bytes())?;

            let dir_size = Self::dir_size(self.path.clone());
            if dir_size > 10240 {
                self.compact()?;
            }

            return Ok(());
        }
        println!("Key not found");
        Err(KvsError::KeyNotFound)
    }

    /// Open the KvStore at a given path.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into().join("store.txt");
        if !path.exists() {
            File::create(path.clone())?;
        }

        let writer_file = File::options().read(true).append(true).open(path.clone())?;
        let writer = BufWriter::new(writer_file.try_clone()?);
        let reader_file = File::options().read(true).open(path.clone())?;
        let reader = BufReader::new(reader_file);

        let mut index = HashMap::new();
        let replay_reader = BufReader::new(writer_file);
        let offset = Self::replay(replay_reader, &mut index);
        Ok(KvStore {
            path,
            reader,
            writer,
            index,
            offset,
        })
    }

    // Replay from file, add (key, offset) pairs to the memory.
    fn replay(reader: BufReader<File>, index: &mut HashMap<String, u64>) -> u64 {
        let mut stream = Deserializer::from_reader(reader).into_iter::<Command>();
        let mut offset = 0;
        while let Some(command) = stream.next() {
            if let Ok(command) = command {
                match command {
                    Command::Set { key, .. } => {
                        index.insert(key, offset);
                    }
                    Command::Rm { key, .. } => {
                        index.remove(&key);
                    }
                    _ => {}
                };
                offset = stream.byte_offset() as u64;
            }
        }
        offset
    }

    // Calculate the size of the directory
    fn dir_size(path: impl Into<PathBuf>) -> u64 {
        let entries = WalkDir::new(path.into()).into_iter();
        let len: walkdir::Result<u64> = entries
            .map(|res| {
                res.and_then(|entry| entry.metadata())
                    .map(|metadata| metadata.len())
            })
            .sum();
        len.expect("fail to get directory size")
    }

    // Compact log file by creating a new file and then replacing the old file
    fn compact(&mut self) -> Result<()> {
        let path: PathBuf = self.path.parent().unwrap().join("store_new.txt");
        if !path.exists() {
            File::create(path.clone())?;
        }

        let new_file = File::options().read(true).append(true).open(path.clone())?;
        let mut new_writer = BufWriter::new(new_file.try_clone()?);
        for (_, offset) in self.index.iter() {
            self.reader.seek(std::io::SeekFrom::Start(*offset))?;
            let mut stream =
                serde_json::Deserializer::from_reader(&mut self.reader).into_iter::<Command>();
            if let Some(Ok(log)) = stream.next() {
                let log_json = serde_json::to_string(&log)?;
                new_writer.write_all(log_json.as_bytes())?;
            }
        }
        new_writer.flush()?;

        let new_file = File::options().read(true).append(true).open(path.clone())?;
        let replay_reader = BufReader::new(new_file.try_clone()?);
        let mut index = HashMap::new();
        let offset = Self::replay(replay_reader, &mut index);

        // use new file replacing the old file
        std::fs::remove_file(self.path.clone())?;
        std::fs::rename(path.clone(), self.path.clone())?;

        let reader_file = File::options().read(true).open(self.path.clone())?;
        let reader = BufReader::new(reader_file);
        let writer_file = File::options()
            .read(true)
            .append(true)
            .open(self.path.clone())?;
        let writer = BufWriter::new(writer_file.try_clone()?);

        self.reader = reader;
        self.writer = writer;
        self.index = index;
        self.offset = offset;

        Ok(())
    }
}
