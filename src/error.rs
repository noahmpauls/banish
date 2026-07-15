use std::{fmt, io};

pub type Result<T> = std::result::Result<T, BanishError>;

#[derive(Debug)]
pub enum BanishError {
    BadArgs,
    AlreadyBanished(String),
    DomainParse(String),
    HostsFile(io::Error),
    HostsParse(String),
}

impl fmt::Display for BanishError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadArgs => write!(f, "usage: banish <url or domain>"),
            Self::AlreadyBanished(domain) => write!(f, "{} is already banished", domain),
            Self::DomainParse(details) => write!(f, "could not parse domain ({})", details),
            Self::HostsFile(err) => write!(f, "could not read/write hosts file ({})", err),
            Self::HostsParse(details) => write!(f, "could not parse hosts file ({})", details),
        }
    }
}

impl From<io::Error> for BanishError {
    fn from(value: io::Error) -> Self {
        BanishError::HostsFile(value)
    }
}
