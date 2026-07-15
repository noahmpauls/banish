use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::domain::Domain;

const BANISH_START: &str = "### BANISH START";
const BANISH_END: &str = "### BANISH END";

pub fn read_hosts_file() -> Result<String, String> {
    let Ok(contents) = fs::read_to_string("/etc/hosts") else {
        return Err("unable to read /etc/hosts".to_owned());
    };
    Ok(contents)
}

#[derive(Debug)]
pub struct HostsFile {
    before: Vec<String>,
    after: Vec<String>,
    banished: Vec<Domain>,
}

impl HostsFile {
    pub fn parse(contents: &str) -> Result<Self, String> {
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
            let domain = line.split_whitespace().nth(1).unwrap();
            let domain = Domain::parse(&domain).unwrap();
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

    pub fn banish(&mut self, domain: &Domain) -> Result<(), String> {
        if self.is_banished(domain) {
            return Err("domain already banished".to_owned());
        }
        self.banished.push(domain.clone());
        self.banished.sort();
        Ok(())
    }

    pub fn construct(&self) -> String {
        let mut result = String::new();
        for line in self.before.iter() {
            result.push_str(line);
            result.push('\n');
        }

        result.push_str(&format!("{}\n", BANISH_START));
        for domain in self.banished.iter() {
            result.push_str(&format!("0.0.0.0 {}\n", domain.construct_for_hosts()));
        }
        result.push_str(&format!("{}\n", BANISH_END));

        for line in self.after.iter() {
            result.push_str(line);
            result.push('\n');
        }
        result
    }
}

pub fn write_hosts_file(contents: &str) -> Result<(), String> {
    // FIXME: use a proper temp file name
    let temp_file_path = Path::join(env::temp_dir().as_path(), "banishhosts");
    let Ok(mut temp_file) = File::create(&temp_file_path) else {
        return Err("cannot create temporary hosts file".to_owned());
    };
    let Ok(_) = write!(temp_file, "{}", contents) else {
        return Err("failed to write to temporary hosts file".to_owned());
    };
    let Ok(_) = fs::rename(&temp_file_path, "/etc/hosts") else {
        return Err("failed to rename temporary hosts file to /etc/hosts".to_owned());
    };
    Ok(())
}
