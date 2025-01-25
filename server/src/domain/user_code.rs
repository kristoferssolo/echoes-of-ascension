use std::ops::Deref;

use rand::{thread_rng, Rng};
use secrecy::SecretString;

#[derive(Debug)]
pub struct UserCode(SecretString);

impl UserCode {
    pub fn generate() -> Self {
        Self::default()
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
