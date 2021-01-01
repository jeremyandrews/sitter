/// Functions to help with sending and validating email.
use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;
use uuid::Uuid;

use crate::person::{Person, PersonHook, PersonRequest};

pub struct Email;

impl PersonHook for Email {
    /// Validate the syntax of an e-mail address.
    fn validate(&self, person_request: &PersonRequest, _action: &str) -> Result<()> {
        // Compile the regular expression only one time.
        // @TODO: improve the regular expression.
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                ^(?P<login>[^@\s]+)@
                ([[:word:]]+\.)*
                [[:word:]]+$
            "
            )
            .unwrap();
        }
        if !RE.is_match(&person_request.email) {
            return Err(anyhow!("Invalid email address"));
        }

        Ok(())
    }

    /// No preprocessing is done of emails.
    fn prepare(&self, _person_request: &mut PersonRequest, _action: &str) -> Result<()> {
        Ok(())
    }

    /// No prepare_id is done of emails.
    fn prepare_id(&self, _id: &mut Uuid, _action: &str) -> Result<()> {
        Ok(())
    }

    /// No postprocessing is done of emails.
    fn processed(&self, _person: &mut Person, _action: &str) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validate_email() {
        use crate::email::{Email, PersonRequest};
        use crate::person::PersonHook;

        let mut person_request = PersonRequest {
            email: "".to_string(),
            pass: "".to_string(),
        };

        // Simulate invoking a hook.
        let email_object = Email {};

        // Very some valid email formats.
        for email in vec![
            "somebody@example.com",
            "somebody@sub.example.com",
            "somebody@127.0.0.1",
            // @FIXME: all of the following are valid email addresses.
            //"somebody@ex-ample.com",
            //"somebody@0000:0000:0000:0000:0000:0000:0000:0001",
            //"somebody@0:0:0:0:0:0:0:1",
        ] {
            person_request.email = email.to_string();
            assert_eq!(
                Email::validate(&email_object, &person_request, "create").is_ok(),
                true
            );
        }

        // TODO Verify that the following errors return an error.
        for email in vec![
            "no body@example.com",
            "no body@sub.example.com",
            "nobody@127 0.0.1",
            "nobody.example.com",
            "nobody@nobody@example.com",
            "nobody@-example.com",
            "nobody@example-.com",
        ] {
            person_request.email = email.to_string();
            assert_eq!(
                Email::validate(&email_object, &person_request, "create").is_ok(),
                false
            );
        }
    }
}
