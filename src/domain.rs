use std::{cmp::Ordering, fmt::Display};

use crate::error::BanishError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Domain {
    domain: String,
    segments: Vec<String>,
}

impl Domain {
    pub fn parse(url: &str) -> crate::error::Result<Self> {
        // split on protocol separator and take everything after
        let segments: Vec<&str> = url.split("://").collect();
        if segments.len() > 2 {
            return Err(BanishError::DomainParse(
                "input contains multiple protocol separators".to_owned(),
            ));
        }
        let domain_and_path = if segments.len() == 2 {
            segments[1]
        } else {
            segments[0]
        };
        // split on path separator and take everything before
        let domain = domain_and_path.split('/').next().unwrap().to_owned();
        if domain.is_empty() {
            return Err(BanishError::DomainParse(
                "input does not contain domain".to_owned(),
            ));
        }
        for c in domain.chars() {
            if !(c.is_ascii_alphanumeric() || c == '.' || c == '-') {
                return Err(BanishError::DomainParse(
                    "domain contains invalid characters".to_owned(),
                ));
            }
        }
        let segments = domain.split('.').map(|s| s.to_owned()).rev().collect();
        Ok(Self { domain, segments })
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }
}

impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.domain)
    }
}

impl Ord for Domain {
    fn cmp(&self, other: &Self) -> Ordering {
        let segments = self.segments.iter().zip(other.segments.iter());
        segments
            .map(|(a, b)| a.cmp(&b))
            .find(|c| *c != Ordering::Equal)
            .unwrap_or(self.segments.len().cmp(&other.segments.len()))
    }
}

impl PartialOrd for Domain {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tld() {
        let input = "com";
        let output = Domain::parse(input).unwrap();
        assert_eq!("com", output.domain);
    }

    #[test]
    fn bare_domain() {
        let input = "www.com";
        let output = Domain::parse(input).unwrap();
        assert_eq!("www.com", output.domain);
    }

    #[test]
    fn domain_with_protocol() {
        let input = "http://test";
        let output = Domain::parse(input).unwrap();
        assert_eq!("test", output.domain);
    }

    #[test]
    fn domain_with_trailing_slash() {
        let input = "test.com/";
        let output = Domain::parse(input).unwrap();
        assert_eq!("test.com", output.domain);
    }

    #[test]
    fn domain_with_path() {
        let input = "test/a/b/c";
        let output = Domain::parse(input).unwrap();
        assert_eq!("test", output.domain);
    }

    #[test]
    fn domain_with_protocol_and_path() {
        let input = "https://test/a/b/c/";
        let output = Domain::parse(input).unwrap();
        assert_eq!("test", output.domain);
    }

    #[test]
    fn absolute_path() {
        let input = "/thing";
        let output = Domain::parse(input);
        assert!(output.is_err());
    }

    #[test]
    fn invalid_domain_chars() {
        let input = "https://abc.X-Y-Z.%.123/test";
        let output = Domain::parse(input);
        assert!(output.is_err());
    }

    #[test]
    fn domain_with_multiple_protocols() {
        let input = "http://test://com/a/b/c/";
        let output = Domain::parse(input);
        assert!(output.is_err());
    }
}
