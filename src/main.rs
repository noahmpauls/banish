use std::{
    env,
    io::{Write, stdin, stdout},
    process::ExitCode,
};

use banish::{
    domain::Domain,
    error::BanishError,
    hostsfile::{HostsFile, read_hosts_file, write_hosts_file},
};

fn main() -> ExitCode {
    match main_with_error() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {}", err);
            ExitCode::FAILURE
        }
    }
}

fn main_with_error() -> banish::error::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(BanishError::BadArgs);
    }
    let url = &args[1];
    let domain = Domain::parse(url)?;

    // if there is no /etc/hosts file
    // - fatal error, don't try to create it
    let hosts_file_contents = read_hosts_file()?;

    // parse /etc/hosts file
    let mut hosts_file = HostsFile::parse(&hosts_file_contents)?;

    // if domain already banished, return early
    if hosts_file.is_banished(&domain) {
        return Err(BanishError::AlreadyBanished(domain.to_string()));
    }

    // if we need to process the url to extract a domain,
    // confirm the domain with the user first
    if domain.domain() != url {
        confirm_domain(&domain);
    }

    // add domain to list
    hosts_file.banish(&domain);

    // write /etc/hosts file
    write_hosts_file(&hosts_file.construct())?;

    // print and return
    println!("{} banished.", domain);
    Ok(())
}

fn confirm_domain(domain: &Domain) {
    print!("Will banish {}. Press Enter to proceed.", domain);
    let _ = stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
}
