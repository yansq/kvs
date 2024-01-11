use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvsError {
    #[error(transparent)]
    Serialize(#[from] serde_json::Error),
    #[error("unknown data store error")]
    Unknown,
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("invalid log data: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Key not found")]
    KeyNotFound,
}
