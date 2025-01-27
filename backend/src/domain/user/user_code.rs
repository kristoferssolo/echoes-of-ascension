use argon2::Argon2;
use password_hash::SaltString;
use std::ops::Deref;

use rand::{rngs::OsRng, thread_rng, Rng};
use secrecy::{ExposeSecret, SecretString};

use super::error::UserError;

#[derive(Debug, Clone)]
pub struct UserCode(SecretString);

impl UserCode {
    pub fn hash(&self) -> Result<String, UserError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let mut output_key_material = [0u8; 32];
        argon2
            .hash_password_into(
                self.expose_secret().as_bytes(),
                salt.as_str().as_bytes(),
                &mut output_key_material,
            )
            .map_err(|e| UserError::HashingError(e.to_string()))?;
        Ok(format!(
            "{}${}",
            salt.as_str(),
            hex::encode(output_key_material)
        ))
    }

    pub fn verify(stored: &str, code: &str) -> Result<bool, UserError> {
        let argon2 = Argon2::default();

        // Split stored value into salt and hash
        let parts: Vec<&str> = stored.split('$').collect();
        if parts.len() != 2 {
            return Err(UserError::HashingError("Invalid hash format".to_string()));
        }

        let salt = parts[0];
        let stored_hash =
            hex::decode(parts[1]).map_err(|e| UserError::HashingError(e.to_string()))?;

        let mut output = [0u8; 32];
        argon2
            .hash_password_into(code.as_bytes(), salt.as_bytes(), &mut output)
            .map_err(|e| UserError::HashingError(e.to_string()))?;

        Ok(output.as_slice() == stored_hash.as_slice())
    }
}

impl Default for UserCode {
    fn default() -> Self {
        let mut rng = thread_rng();

        let code = (0..16)
            .map(|_| rng.gen_range(0..10).to_string())
            .collect::<String>();

        Self(code.into())
    }
}

impl Deref for UserCode {
    type Target = SecretString;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
