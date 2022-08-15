use argon2::{Argon2, PasswordHasher};

pub fn password_to_kdf(
    password: &str,
) -> Result<argon2::PasswordHash, argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), "salt")
}
