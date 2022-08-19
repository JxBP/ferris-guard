use std::fs::File;

use ferris_guard::{
    crypto::ArgonAESProvider,
    storage::{Entry, Storage},
};

const STORAGE_PATH: &str = "/tmp/my_storage.dat";
const DUMMY_PASSWORD: &str = "dummy";

fn main() -> anyhow::Result<()> {
    let provider = ArgonAESProvider {};
    {
        let mut storage = Storage::new(provider)?;
        storage.db.entries.push(Entry {
            name: "EPIC NAME".to_string(),
            email: "BEST EMAIL EVER".to_string(),
            password: "SUPER SERCRET PASSWORD".to_string(),
            tags: Vec::new(),
        });
        storage.save(File::create(STORAGE_PATH)?, DUMMY_PASSWORD)?;
    }

    let storage = Storage::open(File::open(STORAGE_PATH)?, DUMMY_PASSWORD, provider)?;
    println!("{:?}", storage.db);
    Ok(())
}
