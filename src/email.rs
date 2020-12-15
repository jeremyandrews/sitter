/// Functions to help with sending and validating email.
use lazy_static::lazy_static;
use regex::Regex;

/// Validate the syntax of an e-mail address.
/// @TODO: make this pluggable.
pub fn is_valid_email_address(address: &str) -> bool {
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
    RE.is_match(address)
}

#[cfg(test)]
mod tests {
    #[test]
    fn validate_email() {
        use crate::email::is_valid_email_address;
        // Valid email address.
        assert_eq!(is_valid_email_address("somebody@example.com"), true);
        // Invalid email address.
        assert_eq!(is_valid_email_address("no body@example.com"), false);
        // Valid email address using subdomain.
        assert_eq!(is_valid_email_address("somebody@sub.example.com"), true);
        // Invalid email address using subdomain.
        assert_eq!(is_valid_email_address("no body@sub.example.com"), false);
        // Valid email address using IP address.
        assert_eq!(is_valid_email_address("somebody@127.0.0.1"), true);
        // Ivalid email address using invalid IP address.
        assert_eq!(is_valid_email_address("somebody@127 0.0.1"), false);
        // @TODO @FIXME Valid email address using IPv6 address.
        //assert_eq!(is_valid_email_address("somebody@0000:0000:0000:0000:0000:0000:0000:0001"), true);
        //assert_eq!(is_valid_email_address("somebody@0:0:0:0:0:0:0:1"), true);
        // Invalid email address without an @ symbol.
        assert_eq!(is_valid_email_address("nobody.example.com"), false);
        // Invalid email address with multiple @ symbols.
        assert_eq!(is_valid_email_address("nobody@nobody@example.com"), false);
    }
}
