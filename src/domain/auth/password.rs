use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::utils::error::ApiError;

pub fn hash_password(password: String) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let password = password.as_bytes();
    let password_argon = Argon2::default();
    match password_argon.hash_password(&password, &salt) {
        Ok(password) => Ok(password.to_string()),
        Err(e) => Err(ApiError::internal_msg(format!("encryption error; {e}"))),
    }
}

pub fn verify_password(hash_password: &str, password: &str) -> Result<bool, ApiError> {
    let password_argon = Argon2::default();

    let parsed_hash = PasswordHash::new(&hash_password)
        .map_err(|e| ApiError::internal_msg(format!("decryption error: {e}")))?;
    let password = password.as_bytes();
    Ok(password_argon
        .verify_password(password, &parsed_hash)
        .is_ok())
}

