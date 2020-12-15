use anyhow::{anyhow, Result};
use argon2::{self, Config, ThreadMode, Variant, Version};
use rand::Rng;

use crate::person::{Person, PersonHooks, PersonRequest};

pub struct Password;
impl Password {
    // Load secret salt from environment.
    fn random_salt() -> [u8; 24] {
        rand::thread_rng().gen::<[u8; 24]>()
    }
}

impl PersonHooks for Password {
    /// Simple password validation function, requires passwords be at
    /// least 8 characters in length.
    fn validate(person: &PersonRequest) -> Result<()> {
        if person.pass.len() < 8 {
            return Err(anyhow!(
                "password too short, must be at least 8 characters long"
            ));
        }
        Ok(())
    }

    fn presave(person_request: &mut PersonRequest) -> Result<()> {
        let plain_text = person_request.pass.clone().into_bytes();
        let secret_salt = Password::random_salt();
        let config = Config {
            // Argon2id is a hybrid of Argon2i and Argon2d, using a combination
            // of data-depending and data-independent memory accesses, which
            // gives some of Argon2i's resistance to side-channel cache timing
            // attacks and much of Argon2d's resistance to GPU cracking attacks.
            variant: Variant::Argon2id,
            // Version13 is the latest algorithm version.
            version: Version::Version13,
            // Per-lane memory allocation in KB.
            mem_cost: 32768,
            // How many iterations to make.
            time_cost: 3,
            // How many parallel lanes of memory to fill.
            lanes: 4,
            // Allow parallel processing.
            thread_mode: ThreadMode::Parallel,
            secret: &[],
            ad: &[],
            hash_length: 32,
        };
        person_request.pass = argon2::hash_encoded(&plain_text, &secret_salt, &config)?;
        Ok(())
    }

    /// No postprocessing is done of passwords.
    fn postsave(_person: &mut Person, _action: &str) -> Result<()> {
        Ok(())
    }
}
