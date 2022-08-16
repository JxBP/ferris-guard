use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("could not open storage")]
    OpenError(#[from] std::io::Error),

    #[error("could not (de)serialize database")]
    SerializationError(#[from] ron::error::Error),
}
