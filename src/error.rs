use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvsError {
    #[error("unknown data store error")]
    Unknown,
}
