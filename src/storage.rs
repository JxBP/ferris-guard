use crate::{crypto::CryptoProvider, errors::StorageError};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;

pub struct Storage<T: CryptoProvider + Copy> {
    pub db: Database,
    path: String,
    provider: T,
}

impl<T: CryptoProvider + Copy> Storage<T> {
    /// Creates a new data storage with the given password, this does not read data from the file.
    pub fn new(path: &str, provider: T) -> Result<Storage<T>, StorageError> {
        Ok(Storage {
            db: Database {
                tags: vec![],
                entries: vec![],
            },
            path: path.to_string(),
            provider,
        })
    }

    /// Opens an existing storage with the given password.
    pub fn open(path: &str, password: &str, provider: T) -> Result<Storage<T>, StorageError> {
        let content = std::fs::read_to_string(path)?;

        let raw_db = String::from_utf8(provider.decrypt(password, content.as_bytes()))?;
        let db: Database = ron::from_str(&raw_db)?;

        Ok(Storage {
            db,
            path: path.to_string(),
            provider,
        })
    }

    /// Saves and encrypts the database with the given password.
    pub fn save(self, password: &str) -> Result<(), StorageError> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(self.path)?;
        let serialized_db = ron::to_string(&self.db)?;
        file.write_all(&self.provider.encrypt(password, serialized_db.as_bytes()))?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    id: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub name: String,
    pub email: String,
    pub password: String,
    pub tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub tags: Vec<Tag>,
    pub entries: Vec<Entry>,
}
