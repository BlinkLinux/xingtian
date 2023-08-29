// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use dns_lookup::getnameinfo;
use std::fs;
use std::net::{IpAddr, SocketAddr};

use crate::error::Error;

pub struct DnsServer {
    pub ip: String,
    pub hostname: String,
}

/// # Errors
/// Returns error if failed to parse resolv file, or failed to retrieve dns.
pub fn get_list() -> Result<Vec<DnsServer>, Error> {
    const FILE: &str = "/etc/resolv.conf";
    let content = fs::read_to_string(FILE).map_err(|err| Error::IoError(FILE, err))?;
    let mut list = Vec::new();

    for line in content.lines() {
        let mut iter = line.split_ascii_whitespace();
        if iter.next() == Some("nameserver") {
            let domain = iter
                .next()
                .ok_or_else(|| Error::ParseFile(FILE, "Invalid nameserver"))?
                .trim();
            let hostname = get_hostname_by_ip(domain)?;
            list.push(DnsServer {
                ip: domain.to_owned(),
                hostname,
            });
        }
    }
    Ok(list)
}

/// # Errors
/// Returns error if failed to parse addr as ip or failed to retrieve dns.
pub fn get_hostname_by_ip(addr: &str) -> Result<String, Error> {
    let ip: IpAddr = addr.parse()?;
    let port = 53;
    let socket: SocketAddr = (ip, port).into();

    let (name, _service) =
        getnameinfo(&socket, 0).map_err(|err| Error::DnsError(addr.to_owned(), err))?;
    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::{get_hostname_by_ip, get_list};

    #[test]
    fn test_get_list() {
        let list = get_list();
        assert!(list.is_ok());
        assert!(list.unwrap().len() >= 1);
    }

    #[test]
    fn test_get_hostname_by_ip() {
        let hostname = get_hostname_by_ip("8.8.8.8");
        assert!(hostname.is_ok());
        assert_eq!(hostname.unwrap(), "dns.google");
    }
}
