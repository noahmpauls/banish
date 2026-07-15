use std::{
    env,
    io::{Write, stdin, stdout},
};

use banish::{
    domain::Domain,
    hostsfile::{HostsFile, read_hosts_file, write_hosts_file},
};

fn main() {
    let url = env::args().nth(1).expect("url required");
    let Ok(domain) = Domain::parse(&url) else {
        // FIXME: return an Err Result
        eprintln!("Unable to parse domain from input.");
        return;
    };

    // if there is no /etc/hosts file
    // - fatal error, don't try to create it
    let Ok(hosts_file_contents) = read_hosts_file() else {
        // FIXME: return an Err Result
        eprintln!("Unable to retrieve contents of /etc/hosts file.");
        return;
    };

    // parse /etc/hosts file
    let Ok(mut hosts_file) = HostsFile::parse(&hosts_file_contents) else {
        // FIXME: return an Err Result
        eprintln!("Unable to parse /etc/hosts file.");
        return;
    };

    // if domain already banished, return early
    if hosts_file.is_banished(&domain) {
        // FIXME: return an Err Result
        eprintln!("Domain is already banished.");
        return;
    }

    // if we need to process the url to extract a domain,
    // confirm the domain with the user first
    if domain.domain() != url {
        confirm_domain(&domain);
    }

    // add domain to list
    let Ok(_) = hosts_file.banish(&domain) else {
        // FIXME: return an Err Result
        eprintln!("Unable to banish domain.");
        return;
    };

    // write /etc/hosts file
    let Ok(_) = write_hosts_file(&hosts_file.construct()) else {
        // FIXME: return an Err Result
        eprintln!("Unable to write /etc/hosts file.");
        return;
    };

    // print and return
    println!("{} banished.", domain);
}

fn confirm_domain(domain: &Domain) {
    print!("Will banish {}. Press Enter to proceed.", domain);
    let _ = stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
}
