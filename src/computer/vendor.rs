// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Clone)]
pub struct Vendor {
    pub id: &'static str,
    pub name: &'static str,
    pub url: &'static str,
}

impl Vendor {
    #[must_use]
    pub const fn new(id: &'static str, name: &'static str, url: &'static str) -> Self {
        Self { id, name, url }
    }
}

#[rustfmt::skip]
const VENDOR_LIST: &[Vendor] = &[
    Vendor::new("ATI", "ATI Technologies", "https://www.amd.com"),
    Vendor::new("nVidia", "nVidia", "https://www.nvidia.com"),
    Vendor::new("NVidia", "nVidia", "https://www.nvidia.com"),
    Vendor::new("3Com", "3Com", "https://www.hpe.com"),
    Vendor::new("Intel", "Intel", "https://www.intel.com"),
    Vendor::new("Cirrus Logic", "Cirrus Logic", "https://www.cirrus.com"),
    Vendor::new("VIA Technologies", "VIA Technologies", "https://www.viatech.com"),
    Vendor::new("VIA", "VIA Technologies", "https://www.viatech.com"),
    Vendor::new("hp", "Hewlett-Packard", "https://www.hp.com"),
    Vendor::new("NEC Corporation", "NEC Coporation", "https://www.nec.com"),
    Vendor::new("MAXTOR", "MAXTOR", "https://www.seagate.com"),
    Vendor::new("SAMSUNG", "SAMSUNG", "https://www.samsung.com"),
    Vendor::new("PIONEER", "PIONEER", "https://www.pioneer.eur"),
    Vendor::new("PLEXTOR", "PLEXTOR", "https://www.goplextor.com"),
    Vendor::new("Realtek Semiconductor", "Realtek", "https://www.realtek.com"),
    Vendor::new("TOSHIBA", "TOSHIBA", "https://www.toshiba.com"),
    Vendor::new("LITE-ON", "LITE-ON", "https://www.liteon.com"),
    Vendor::new("WDC", "Western Digital", "https://www.westerndigital.com"),
    Vendor::new("HL-DT-ST", "LG Electronics", "https://www.lg.com"),
    Vendor::new("ST", "SEAGATE", "https://www.seagate.com"),
    Vendor::new("Lexmark", "Lexmark", "https://www.lexmark.com"),
    Vendor::new("_NEC", "NEC Corporation", "https://www.necam.com"),
    Vendor::new("Creative Labs", "Creative Labs", "https://creative.com"),
    Vendor::new("Brooktree", "Conexant", "https://www.synaptics.com"),
    Vendor::new("Atheros", "Atheros Communications", "https://www.qualcomm.com"),
    Vendor::new("MATSHITA", "Panasonic", "https://panasonic.com"),
    Vendor::new("Silicon Image", "Silicon Image", "https://www.latticesemi.com"),
    Vendor::new("Silicon Integrated Image", "Silicon Image", "https://www.latticesemi.com"),
    Vendor::new("KYE", "KYE Systems", "https://global.geniusnet.com"),
    Vendor::new("Broadcom", "Broadcom", "https://www.broadcom.com"),
    Vendor::new("Apple", "Apple", "https://www.apple.com"),
    Vendor::new("IBM", "IBM", "https://www.ibm.com"),
    Vendor::new("Dell", "Dell Computer", "https://www.dell.com"),
    Vendor::new("Logitech", "Logitech International", "https://www.logitech.com"),
    Vendor::new("FUJITSU", "Fujitsu", "https://www.fujitsu.com"),
    Vendor::new("CDU", "Sony", "https://www.sony.com"),
    Vendor::new("SanDisk", "SanDisk", "https://www.westerndigital.com"),
    Vendor::new("ExcelStor", "ExcelStor Technology", ""),
    Vendor::new("D-Link", "D-Link", "https://www.dlink.com"),
    Vendor::new("Giga-byte", "Gigabyte Technology", "https://www.gigabyte.com"),
    Vendor::new("Gigabyte", "Gigabyte Technology", "https://www.gigabyte.com"),
    Vendor::new("C-Media", "C-Media Electronics", "https://www.cmedia.com.tw"),
    Vendor::new("Avermedia", "AVerMedia Technologies", "https://www.averusa.com"),
    Vendor::new("Philips", "Philips", "https://www.philips.com"),
    Vendor::new("RaLink", "Ralink Technology", "https://www.mediatek.com"),
    Vendor::new("Siemens", "Siemens AG", "https://www.siemens.com"),
    Vendor::new("HP", "Hewlett-Packard", "https://www.hp.com"),
    Vendor::new("Hewlett-Packard", "Hewlett-Packard", "https://www.hp.com"),
    Vendor::new("TEAC", "TEAC America", "https://www.teac.co.jp"),
    Vendor::new("Microsoft", "Microsoft", "https://www.microsoft.com"),
    Vendor::new("Memorex", "Memorex Products", "https://www.memorex.com"),
    Vendor::new("eMPIA", "eMPIA Technology", "https://www.empiatech.com"),
    Vendor::new("Canon", "Canon", "https://www.canon.com"),
    Vendor::new("A4Tech", "A4tech", "https://www.a4tech.com"),
    Vendor::new("ALCOR", "Alcor", "https://www.alcor.org"),
    Vendor::new("Vimicro", "Vimicro", "http://www.vimicro.com"),
    Vendor::new("OTi", "Ours Technology", "https://www.oti.com.tw"),
    Vendor::new("BENQ", "BenQ", "https://www.benq.com"),
    Vendor::new("Acer", "Acer", "https://www.acer.com"),
    Vendor::new("QUANTUM", "Quantum", "https://www.quantum.com"),
    Vendor::new("Kingston", "Kingston", "https://www.kingston.com"),
    Vendor::new("Chicony", "Chicony", "https://www.chicony.com"),
    Vendor::new("Genius", "Genius", "https://genius.ru"),
    /* BIOS manufacturers */
    Vendor::new("American Megatrends", "American Megatrends", "https://www.ami.com"),
    Vendor::new("Award", "Award Software International", "https://www.award-bios.com"),
    Vendor::new("Phoenix", "Phoenix Technologies", "https://www.phoenix.com"),
    /* x86 vendor strings */
    Vendor::new("AMDisbetter!", "Advanced Micro Devices", "https://www.amd.com"),
    Vendor::new("AuthenticAMD", "Advanced Micro Devices", "https://www.amd.com"),
    Vendor::new("CentaurHauls", "VIA (formerly Centaur Technology)", "https://www.via.tw"),
    Vendor::new("CyrixInstead", "Cyrix", "https://www.amd.com"),
    Vendor::new("GenuineIntel", "Intel", "https://www.intel.com"),
    Vendor::new("TransmetaCPU", "Transmeta", ""),
    Vendor::new("GenuineTMx86", "Transmeta", ""),
    Vendor::new("Geode by NSC", "National Semiconductor", ""),
    Vendor::new("NexGenDriven", "NexGen", ""),
    Vendor::new("RiseRiseRise", "Rise Technology", ""),
    Vendor::new("SiS SiS SiS", "Silicon Integrated Systems", ""),
    Vendor::new("UMC UMC UMC", "United Microelectronics Corporation", "https://www.umc.com"),
    Vendor::new("VIA VIA VIA", "VIA", "https://www.via.tw"),
    Vendor::new("Vortex86 SoC", "DMP Electronics", "https://www.vortex86.com"),
    /* x86 VM vendor strings */
    Vendor::new("KVMKVMKVM", "KVM", "https://www.linux-kvm.org"),
    Vendor::new("Microsoft Hv", "Microsoft Hyper-V", "https://www.microsoft.com"),
    Vendor::new("lrpepyh vr", "Parallels", ""),
    Vendor::new("VMwareVMware", "VMware", "https://www.vmware.com"),
    Vendor::new("XenVMMXenVMM", "Xen HVM", ""),
];

#[must_use]
pub fn get_name(id: &str) -> Option<&'static str> {
    for vendor in VENDOR_LIST {
        if id.contains(vendor.id) {
            return Some(vendor.name);
        }
    }
    None
}

#[must_use]
pub fn get_url(id: &str) -> Option<&'static str> {
    for vendor in VENDOR_LIST {
        if id.contains(vendor.id) {
            return Some(vendor.url);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{get_name, get_url};

    #[test]
    fn test_get_name() {
        assert_eq!(
            get_name("Realtek Semiconductor Co., Ltd. RTL8821CE"),
            Some("Realtek")
        );
        assert_eq!(
            get_name("Hewlett-Packard Company RTL8821CE"),
            Some("Hewlett-Packard")
        );
    }

    #[test]
    fn test_get_url() {
        assert_eq!(
            get_url("Realtek Semiconductor Co., Ltd. RTL8821CE"),
            Some("www.realtek.com.tw")
        );
        assert_eq!(
            get_url("Hewlett-Packard Company RTL8821CE"),
            Some("www.hp.com")
        );
    }
}
