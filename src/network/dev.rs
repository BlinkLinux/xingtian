// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

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
    pub ip: String,
    pub mac: String,
    pub mask: String,
    pub broadcast: String,

    pub mtu: i32,
    pub sent: i64,
    pub received: i64,
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
