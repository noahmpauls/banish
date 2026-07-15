use std::{fs, io::Write};

use tempfile::NamedTempFile;

use crate::domain::Domain;

const BANISH_START: &str = "### BANISH START";
const BANISH_END: &str = "### BANISH END";

pub fn read_hosts_file() -> crate::error::Result<String> {
    let contents = fs::read_to_string("/etc/hosts")?;
    Ok(contents)
}

#[derive(Debug)]
pub struct HostsFile {
    before: Vec<String>,
    after: Vec<String>,
    banished: Vec<Domain>,
}

impl HostsFile {
    pub fn parse(contents: &str) -> crate::error::Result<Self> {
        let mut lines = contents.lines();
        let mut before = vec![];
        for line in lines.by_ref() {
            if line.starts_with(BANISH_START) {
                break;
            }
            before.push(line.to_owned());
        }
        let mut banished = vec![];
        for line in lines.by_ref() {
            if line.starts_with(BANISH_END) {
                break;
            }
            // FIXME: assumes expected formatting
            let domain = line.split_whitespace().nth(1).unwrap();
            let domain = Domain::parse(domain).unwrap();
            banished.push(domain);
        }
        let after = lines.map(|s| s.to_owned()).collect();
        Ok(Self {
            before,
            after,
            banished,
        })
    }

    pub fn is_banished(&self, domain: &Domain) -> bool {
        self.banished.iter().any(|s| s == domain)
    }

    pub fn banish(&mut self, domain: &Domain) -> bool {
        if self.is_banished(domain) {
            return false;
        }
        self.banished.push(domain.clone());
        self.banished.sort();
        true
    }

    pub fn construct(&self) -> String {
        let mut result = String::new();
        for line in self.before.iter() {
            result.push_str(line);
            result.push('\n');
        }

        result.push_str(&format!("{}\n", BANISH_START));
        let pad = self
            .banished
            .iter()
            .map(|d| d.domain().chars().count())
            .max()
            .unwrap();
        for domain in self.banished.iter() {
            result.push_str(&format!("0.0.0.0 {:>pad$}\n", domain.domain(), pad = pad));
        }
        result.push_str(&format!("{}\n", BANISH_END));

        for line in self.after.iter() {
            result.push_str(line);
            result.push('\n');
        }
        result
    }
}

pub fn write_hosts_file(contents: &str) -> crate::error::Result<()> {
    let mut temp_file = NamedTempFile::new()?;
    write!(temp_file, "{}", contents)?;
    fs::rename(&temp_file, "/etc/hosts")?;
    Ok(())
}
