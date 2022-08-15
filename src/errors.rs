use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("could not open storage")]
    OpenError(#[from] std::io::Error),
}
