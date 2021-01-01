/// Provide a logger.
use log::info;
use uuid::Uuid;

use anyhow::Result;

use crate::person::{Person, PersonHook, PersonRequest};

pub struct Logger;

impl PersonHook for Logger {
    /// No logger event for validate.
    fn validate(&self, _person_request: &PersonRequest, _action: &str) -> Result<()> {
        Ok(())
    }

    /// No logger event for validate.
    fn prepare(&self, _person_request: &mut PersonRequest, _action: &str) -> Result<()> {
        Ok(())
    }

    /// No logger event for prepare_id.
    fn prepare_id(&self, _id: &mut Uuid, _action: &str) -> Result<()> {
        Ok(())
    }

    /// Log activity in postsave.
    fn processed(&self, person: &mut Person, action: &str) -> Result<()> {
        info!("{} person '{}'", action, person.email);

        Ok(())
    }
}
