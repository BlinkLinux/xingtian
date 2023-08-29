// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs;

use crate::error::Error;

#[derive(Debug, Default, Clone)]
pub struct Arp {
    pub ip: String,
    pub hw_type: String,
    pub flags: String,
    pub hw_address: String,
    pub mask: String,
    /// network interface
    pub device: String,
}

/// # Errors
/// Returns error if failed to parse arp file.
pub fn get_list() -> Result<Vec<Arp>, Error> {
    const FILE: &str = "/proc/net/arp";
    let content = fs::read_to_string(FILE).map_err(|err| Error::IoError(FILE, err))?;

    let mut list = Vec::new();
    for line in content.lines() {
        if line.starts_with("Iface") {
            // Found header line.
            let parts = line.split('\t');
            if parts.count() != 6 {
                return Err(Error::ParseFile(
                    FILE,
                    "Failed to parse arp file, expect 6 parts",
                ));
            }
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        let parts = line.split('\t');
        let mut arp = Arp::default();
        for (index, part) in parts.enumerate() {
            match index {
                0 => arp.ip = part.to_owned(),
                1 => arp.hw_type = part.to_owned(),
                2 => arp.flags = part.to_owned(),
                3 => arp.hw_address = part.to_owned(),
                4 => arp.mask = part.to_owned(),
                5 => arp.device = part.to_owned(),
                _ => {
                    return Err(Error::ParseFile(
                        FILE,
                        "Failed to parse arp file, expect 6 parts",
                    ));
                }
            }
        }
        list.push(arp);
    }

    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::get_list;

    #[test]
    fn test_get_list() {
        let list = get_list();
        assert!(list.is_ok());
        assert!(list.unwrap().len() >= 1);
    }
}
