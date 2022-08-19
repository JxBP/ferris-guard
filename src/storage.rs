use crate::{crypto::CryptoProvider, errors::StorageError};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};

pub struct Storage<T: CryptoProvider + Copy> {
    pub db: Database,
    provider: T,
}

impl<T: CryptoProvider + Copy> Storage<T> {
    /// Creates an empty data storage.
    pub fn new(provider: T) -> Result<Storage<T>, StorageError> {
        Ok(Storage {
            db: Database {
                tags: vec![],
                entries: vec![],
            },
            provider,
        })
    }

    /// Opens and reads an existing storage using the given password.
    pub fn open<R: Read>(
        mut source: R,
        password: &str,
        provider: T,
    ) -> Result<Storage<T>, StorageError> {
        // Might add a parameter later to specify database size so we can specify the vector's
        // capacity. E.g. if the function user knows the size by looking at file metadata.
        let mut content = Vec::new();
        source.read_to_end(&mut content)?;

        let raw_db = String::from_utf8(provider.decrypt(password, &content))?;
        let db: Database = ron::from_str(&raw_db)?;

        Ok(Storage { db, provider })
    }

    /// Encrypts the data storage and writes it to the filesystem using the given password.
    pub fn save<W: Write>(self, mut out: W, password: &str) -> Result<(), StorageError> {
        let serialized_db = ron::to_string(&self.db)?;
        out.write_all(&self.provider.encrypt(password, serialized_db.as_bytes()))?;
        Ok(())
    }

    pub fn save_to_file(self, path: &str, password: &str) -> Result<(), StorageError> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;

        self.save(file, password)?;
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
