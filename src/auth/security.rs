use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use rand_core::OsRng;

/// Hashes a password using Argon2id.
///
/// # Arguments
///
/// * `password` - The plain text password to hash.
///
/// # Returns
///
/// * `Result<String, Box<dyn std::error::Error>>` - The hashed password or an error.
pub async fn hash_password(password: String) -> Result<String, Box<dyn std::error::Error>> {
    let salt = SaltString::generate(&mut OsRng);

    // Use Argon2id with default parameters
    let argon2 = Argon2::default();

    // Hash the password to a PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    Ok(password_hash)
}

/// Verifies a password against a stored hash using Argon2id.
///
/// # Arguments
///
/// * `stored_hash` - The stored password hash.
/// * `password` - The plain text password to verify.
///
/// # Returns
///
/// * `Result<bool, Box<dyn std::error::Error>>` - `true` if the password is valid, `false` otherwise.
pub async fn verify_password(
    stored_hash: &str,
    password: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Parse the stored PHC string
    let parsed_hash = PasswordHash::new(stored_hash).unwrap();

    // Verify the password against the parsed hash using Argon2id
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
