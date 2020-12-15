/// Provide a logger.
use log::info;

use anyhow::Result;

use crate::person::{Person, PersonHooks, PersonRequest};

pub struct Logger;

impl PersonHooks for Logger {
    /// No logger event for validate.
    fn validate(_person_request: &PersonRequest) -> Result<()> {
        Ok(())
    }

    /// No logger event for validate.
    fn presave(_person_request: &mut PersonRequest) -> Result<()> {
        Ok(())
    }

    /// Log activity in postsave.
    fn postsave(person: &mut Person, action: &str) -> Result<()> {
        info!("'{}' {}", person.name, action);

        Ok(())
    }
}
