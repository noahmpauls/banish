use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

const BANISH_START: &str = "### BANISH START";
const BANISH_END: &str = "### BANISH END";

pub fn extract_domain(input: &str) -> Result<&str, String> {
    // split on protocol separator and take everything after
    let segments: Vec<&str> = input.split("://").collect();
    if segments.len() > 2 {
        return Err("input contains multiple protocol separators".to_owned());
    }
    let domain_and_path = if segments.len() == 2 {
        segments[1]
    } else {
        segments[0]
    };
    // split on path separator and take everything before
    let domain = domain_and_path.split('/').next().unwrap();
    // FIXME: need to validate that remaining segment is a valid domain
    Ok(domain)
}

pub fn get_hosts_file_contents() -> Result<String, String> {
    let Ok(contents) = fs::read_to_string("/etc/hosts") else {
        return Err("unable to read /etc/hosts".to_owned());
    };
    Ok(contents)
}

#[derive(Debug)]
pub struct HostsFile {
    before: Vec<String>,
    after: Vec<String>,
    banished: Vec<String>,
}

impl HostsFile {
    pub fn parse(contents: &str) -> Result<HostsFile, String> {
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
            banished.push(domain.to_owned());
        }
        let after = lines.map(|s| s.to_owned()).collect();
        Ok(HostsFile {
            before,
            after,
            banished,
        })
    }

    pub fn is_banished(&self, domain: &str) -> bool {
        self.banished.iter().any(|s| s == domain)
    }

    pub fn banish(&mut self, domain: &str) -> Result<(), String> {
        if self.is_banished(domain) {
            return Err("domain already banished".to_owned());
        }
        self.banished.push(domain.to_owned());
        // FIXME: need to sort with initial www excluded
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
            result.push_str(&format!("0.0.0.0 {}\n", domain));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tld() {
        let input = "com";
        let output = extract_domain(input).unwrap();
        assert_eq!("com", output);
    }

    #[test]
    fn bare_domain() {
        let input = "www.com";
        let output = extract_domain(input).unwrap();
        assert_eq!("www.com", output);
    }

    #[test]
    fn domain_with_protocol() {
        let input = "http://test";
        let output = extract_domain(input).unwrap();
        assert_eq!("test", output);
    }

    #[test]
    fn domain_with_trailing_slash() {
        let input = "test.com/";
        let output = extract_domain(input).unwrap();
        assert_eq!("test.com", output);
    }

    #[test]
    fn domain_with_path() {
        let input = "test/a/b/c";
        let output = extract_domain(input).unwrap();
        assert_eq!("test", output);
    }

    #[test]
    fn domain_with_protocol_and_path() {
        let input = "https://test/a/b/c/";
        let output = extract_domain(input).unwrap();
        assert_eq!("test", output);
    }

    #[test]
    fn domain_with_multiple_protocols() {
        let input = "http://test://com/a/b/c/";
        let output = extract_domain(input);
        assert!(output.is_err());
    }
}
