use crate::{crypto::CryptoProvider, errors::StorageError};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
};

pub struct Storage<T: CryptoProvider + Copy> {
    pub db: Database,
    path: String,
    provider: T,
}

impl<T: CryptoProvider + Copy> Storage<T> {
    /// Creates a new data storage with the given password and opens it.
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
        let mut file = File::open(path)?;
        let mut buf = Vec::with_capacity(file.metadata()?.len() as usize);

        file.read_to_end(&mut buf)?;

        let raw_db = provider.decrypt(password, &buf);
        let db = ron::from_str::<Database>(&String::from_utf8_lossy(&raw_db))?;

        Ok(Storage {
            db,
            path: path.to_string(),
            provider,
        })
    }

    /// Saves and encrypts the database with the given password.
    pub fn save(self, password: &str) -> Result<(), StorageError> {
        let mut file = File::create(self.path)?;
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
