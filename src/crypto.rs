use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
use argon2::password_hash::Output;
use argon2::{Argon2, PasswordHasher};

const SALT: &str = "ThisIsMyEpicSalt";

pub trait CryptoProvider {
    fn encrypt(self, password: &str, payload: &[u8]) -> Vec<u8>;
    fn decrypt(self, password: &str, payload: &[u8]) -> Vec<u8>;
}

#[derive(Clone, Copy)]
pub struct ArgonAESProvider;

impl CryptoProvider for ArgonAESProvider {
    fn encrypt(self, password: &str, payload: &[u8]) -> Vec<u8> {
        let hash = hash_password(password).unwrap();
        let cipher = Aes256Gcm::new_from_slice(hash.as_bytes()).unwrap();
        let nonce = Nonce::from_slice(&hash.as_bytes()[..12]);
        cipher.encrypt(nonce, payload).unwrap()
    }

    fn decrypt(self, password: &str, payload: &[u8]) -> Vec<u8> {
        let hash = hash_password(password).unwrap();
        let cipher = Aes256Gcm::new_from_slice(hash.as_bytes()).unwrap();
        let nonce = Nonce::from_slice(&hash.as_bytes()[..12]);
        cipher.decrypt(nonce, payload).unwrap()
    }
}

fn hash_password(password: &str) -> Option<Output> {
    Argon2::default()
        .hash_password(password.as_bytes(), &SALT)
        .unwrap()
        .hash
}
