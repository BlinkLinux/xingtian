// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs;

use crate::error::Error;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DevType {
    Unknown = 0,
    Loopback = 1,
    Ethernet = 2,
    Wireless = 3,
    Bridge = 4,
    PointToPoint = 5,
    Bluetooth = 6,
    VirtualNetwork = 7,
    Mesh = 8,
}

impl Default for DevType {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Default, Clone)]
pub struct Dev {
    pub interface: String,

    pub received_bytes: i64,
    pub received_packets: i64,
    pub received_errors: i64,
    pub received_drop: i64,
    pub received_fifo: i64,
    pub received_frame: i64,
    pub received_compressed: i64,
    pub received_multicast: i64,

    pub transmit_bytes: i64,
    pub transmit_packets: i64,
    pub transmit_errors: i64,
    pub transmit_drop: i64,
    pub transmit_fifo: i64,
    pub transmit_colls: i64,
    pub transmit_carrier: i64,
    pub transmit_compressed: i64,

    pub ip: String,
    pub mac: String,
    pub mask: String,
    pub broadcast: String,
    pub mtu: i32,

    pub dev_type: DevType,
}

// prefix => DevType
struct NamePair(&'static str, DevType);

const ETH_NAMES: &[NamePair] = &[
    // ethernet
    NamePair("eth", DevType::Ethernet),
    NamePair("tap", DevType::Ethernet),
    NamePair("en", DevType::Ethernet),
    NamePair("net", DevType::Ethernet),
    NamePair("lo", DevType::Loopback),
    // wireless
    NamePair("ath", DevType::Wireless),
    NamePair("wlan", DevType::Wireless),
    NamePair("ra", DevType::Wireless),
    NamePair("wmaster", DevType::Wireless),
    NamePair("wl", DevType::Wireless),
    NamePair("ww", DevType::Wireless),
    // bridget
    NamePair("br", DevType::Bridge),
    NamePair("docker", DevType::Bridge),
    // p2p
    NamePair("ppp", DevType::PointToPoint),
    NamePair("tun", DevType::PointToPoint),
    // bluetooth
    NamePair("bnep", DevType::Bluetooth),
    // virtual
    NamePair("vmnet8", DevType::VirtualNetwork),
    NamePair("vmnet", DevType::VirtualNetwork),
    NamePair("vboxnet", DevType::VirtualNetwork),
    NamePair("ham", DevType::VirtualNetwork),
    NamePair("veth", DevType::VirtualNetwork),
    // mesh
    NamePair("msh", DevType::Mesh),
];

impl DevType {
    #[must_use]
    pub fn from_name(name: &str) -> Self {
        for item in ETH_NAMES {
            if name.starts_with(item.0) {
                return item.1;
            }
        }

        Self::Unknown
    }
}

/// # Errors
/// Returns error if failed to parse dev file.
#[allow(clippy::too_many_lines)]
pub fn get_list() -> Result<Vec<Dev>, Error> {
    const FILE: &str = "/proc/net/dev";
    let content = fs::read_to_string(FILE).map_err(|err| Error::IoError(FILE, err))?;

    let mut list = Vec::new();

    for line in content.lines() {
        // header line
        if line.contains('|') {
            continue;
        }
        let parts = line.split_ascii_whitespace();
        let mut dev = Dev::default();
        for (index, part) in parts.enumerate() {
            match index {
                0 => {
                    dev.interface = part.trim().to_owned();
                    dev.dev_type = DevType::from_name(&dev.interface);
                }
                1 => {
                    dev.received_bytes = part
                        .trim()
                        .parse()
                        .map_err(|_err| Error::ParseFile(FILE, "Failed to parse received bytes"))?;
                }
                2 => {
                    dev.received_packets = part.trim().parse().map_err(|_err| {
                        Error::ParseFile(FILE, "Failed to parse received packets")
                    })?;
                }
                3 => {
                    dev.received_errors = part.trim().parse().map_err(|_err| {
                        Error::ParseFile(FILE, "Failed to parse received errors")
                    })?;
                }
                4 => {
                    dev.received_drop = part
                        .trim()
                        .parse()
                        .map_err(|_err| Error::ParseFile(FILE, "Failed to parse received drop"))?;
                }
                5 => {
                    dev.received_fifo = part
                        .trim()
                        .parse()
                        .map_err(|_err| Error::ParseFile(FILE, "Failed to parse received fifo"))?;
                }
                6 => {
                    dev.received_frame = part
                        .trim()
                        .parse()
                        .map_err(|_err| Error::ParseFile(FILE, "Failed to parse received frame"))?;
                }
                7 => {
                    dev.received_compressed = part.trim().parse().map_err(|_err| {
                        Error::ParseFile(FILE, "Failed to parse received compressed")
                    })?;
                }
                8 => {
                    dev.received_multicast = part.trim().parse().map_err(|_err| {
                        Error::ParseFile(FILE, "Failed to parse received multicast")
                    })?;
                }
                9 => {
                    dev.transmit_bytes = part
                        .trim()
                        .parse()
                        .map_err(|_err| Error::ParseFile(FILE, "Failed to parse transmit bytes"))?;
                }
                10 => {
                    dev.transmit_packets = part.trim().parse().map_err(|_err| {
                        Error::ParseFile(FILE, "Failed to parse transmit packets")
                    })?;
                }
                11 => {
                    dev.transmit_errors = part.trim().parse().map_err(|_err| {
                        Error::ParseFile(FILE, "Failed to parse transmit errors")
                    })?;
                }
                12 => {
                    dev.transmit_drop = part
                        .trim()
                        .parse()
                        .map_err(|_err| Error::ParseFile(FILE, "Failed to parse transmit drop"))?;
                }
                13 => {
                    dev.transmit_fifo = part
                        .trim()
                        .parse()
                        .map_err(|_err| Error::ParseFile(FILE, "Failed to parse transmit fifo"))?;
                }
                14 => {
                    dev.transmit_colls = part
                        .trim()
                        .parse()
                        .map_err(|_err| Error::ParseFile(FILE, "Failed to parse transmit colls"))?;
                }
                15 => {
                    dev.transmit_carrier = part.trim().parse().map_err(|_err| {
                        Error::ParseFile(FILE, "Failed to parse transmit carrier")
                    })?;
                }
                16 => {
                    dev.transmit_compressed = part.trim().parse().map_err(|_err| {
                        Error::ParseFile(FILE, "Failed to parse transmit compressed")
                    })?;
                }
                _ => return Err(Error::ParseFile(FILE, "Too many parts in dev file")),
            }
        }

        get_ip_info(&mut dev)?;

        list.push(dev);
    }

    Ok(list)
}

fn get_ip_info(_dev: &mut Dev) -> Result<(), Error> {
    todo!()
}
