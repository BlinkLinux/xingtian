// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs;

use crate::error::Error;

#[derive(Debug, Default, Clone)]
pub struct Routing {
    pub interface: String,
    pub destination: String,
    pub gateway: String,
    pub flags: String,

    pub refcnt: i32,
    pub used: i32,
    pub metric: i32,
    pub mask: String,

    pub mtu: i32,
    pub window: i32,
    pub irtt: i32,
}

/// # Errors
/// Returns error if failed to parse route file.
pub fn get_list() -> Result<Vec<Routing>, Error> {
    const FILE: &str = "/proc/net/route";
    let content = fs::read_to_string(FILE).map_err(|err| Error::IoError(FILE, err))?;

    let mut list = Vec::new();
    for line in content.lines() {
        if line.starts_with("Iface") {
            // Found header line.
            let parts = line.split_ascii_whitespace();
            if parts.count() != 11 {
                return Err(Error::ParseFile(
                    FILE,
                    "Failed to parse route file, expect 11 parts",
                ));
            }
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        let parts = line.split_ascii_whitespace();
        let mut routing = Routing::default();
        for (index, part) in parts.enumerate() {
            match index {
                0 => routing.interface = part.to_owned(),
                1 => routing.destination = part.to_owned(),
                2 => routing.gateway = part.to_owned(),
                3 => routing.flags = part.to_owned(),
                4 => {
                    routing.refcnt = part
                        .parse()
                        .map_err(|_err| Error::ParseFile(FILE, "Failed to parse refcnt"))?;
                }
                5 => {
                    routing.used = part
                        .parse()
                        .map_err(|_err| Error::ParseFile(FILE, "Failed to parse used"))?;
                }
                6 => {
                    routing.metric = part
                        .parse()
                        .map_err(|_err| Error::ParseFile(FILE, "Failed to parse metric"))?;
                }
                7 => routing.mask = part.to_owned(),
                8 => {
                    routing.mtu = part
                        .parse()
                        .map_err(|_err| Error::ParseFile(FILE, "Failed to parse mtu"))?;
                }
                9 => {
                    routing.window = part
                        .parse()
                        .map_err(|_err| Error::ParseFile(FILE, "Failed to parse window"))?;
                }
                10 => {
                    routing.irtt = part
                        .parse()
                        .map_err(|_err| Error::ParseFile(FILE, "Failed to parse irtt"))?;
                }
                _ => {
                    return Err(Error::ParseFile(
                        FILE,
                        "Failed to parse route file, expect 11 parts",
                    ));
                }
            }
        }
        list.push(routing);
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
