use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvsError {
    #[error("Serialize/Deserialize error")]
    Serialize(#[from] serde_json::Error),
    #[error("unknown data store error")]
    Unknown,
    #[error("IO error")]
    IO(#[from] std::io::Error),
}
