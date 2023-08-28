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
    Vendor::new("ATI", "ATI Technologies", "www.ati.com"),
    Vendor::new("nVidia", "nVidia", "www.nvidia.com"),
    Vendor::new("NVidia", "nVidia", "www.nvidia.com"),
    Vendor::new("3Com", "3Com", "www.3com.com"),
    Vendor::new("Intel", "Intel", "www.intel.com"),
    Vendor::new("Cirrus Logic", "Cirrus Logic", "www.cirrus.com"),
    Vendor::new("VIA Technologies", "VIA Technologies", "www.via.com.tw"),
    Vendor::new("VIA", "VIA Technologies", "www.via.com.tw"),
    Vendor::new("hp", "Hewlett-Packard", "www.hp.com"),
    Vendor::new("NEC Corporation", "NEC Coporation", "www.nec.com"),
    Vendor::new("MAXTOR", "MAXTOR", "www.maxtor.com"),
    Vendor::new("SAMSUNG", "SAMSUNG", "www.samsung.com"),
    Vendor::new("PIONEER", "PIONEER", "www.pioneer-eur.com"),
    Vendor::new("PLEXTOR", "PLEXTOR", "www.plextor.be"),
    Vendor::new("Realtek Semiconductor", "Realtek", "www.realtek.com.tw"),
    Vendor::new("TOSHIBA", "TOSHIBA", "www.toshiba.com"),
    Vendor::new("LITE-ON", "LITE-ON", "www.liteonit.com"),
    Vendor::new("WDC", "Western Digital", "www.wdc.com"),
    Vendor::new("HL-DT-ST", "LG Electronics", "www.lge.com"),
    Vendor::new("ST", "SEAGATE", "www.seagate.com"),
    Vendor::new("Lexmark", "Lexmark", "www.lexmark.com"),
    Vendor::new("_NEC", "NEC Corporation", "www.nec.com"),
    Vendor::new("Creative Labs", "Creative Labs", "www.creative.com"),
    Vendor::new("Brooktree", "Conexant", "www.brooktree.com"),
    Vendor::new("Atheros", "Atheros Communications", "www.atheros.com"),
    Vendor::new("MATSHITA", "Panasonic", "www.panasonic.com"),
    Vendor::new("Silicon Image", "Silicon Image", "www.siliconimage.com"),
    Vendor::new("Silicon Integrated Image", "Silicon Image", "www.siliconimage.com"),
    Vendor::new("KYE", "KYE Systems", "www.genius-kye.com"),
    Vendor::new("Broadcom", "Broadcom", "www.broadcom.com"),
    Vendor::new("Apple", "Apple", "www.apple.com"),
    Vendor::new("IBM", "IBM", "www.ibm.com"),
    Vendor::new("Dell", "Dell Computer", "www.dell.com"),
    Vendor::new("Logitech", "Logitech International", "www.logitech.com"),
    Vendor::new("FUJITSU", "Fujitsu", "www.fujitsu.com"),
    Vendor::new("CDU", "Sony", "www.sony.com"),
    Vendor::new("SanDisk", "SanDisk", "www.sandisk.com"),
    Vendor::new("ExcelStor", "ExcelStor Technology", "www.excelstor.com"),
    Vendor::new("D-Link", "D-Link", "www.dlink.com.tw"),
    Vendor::new("Giga-byte", "Gigabyte Technology", "www.gigabyte.com.tw"),
    Vendor::new("Gigabyte", "Gigabyte Technology", "www.gigabyte.com.tw"),
    Vendor::new("C-Media", "C-Media Electronics", "www.cmedia.com.tw"),
    Vendor::new("Avermedia", "AVerMedia Technologies", "www.aver.com"),
    Vendor::new("Philips", "Philips", "www.philips.com"),
    Vendor::new("RaLink", "Ralink Technology", "www.ralinktech.com"),
    Vendor::new("Siemens", "Siemens AG", "www.siemens.com"),
    Vendor::new("HP", "Hewlett-Packard", "www.hp.com"),
    Vendor::new("Hewlett-Packard", "Hewlett-Packard", "www.hp.com"),
    Vendor::new("TEAC", "TEAC America", "www.teac.com"),
    Vendor::new("Microsoft", "Microsoft", "www.microsoft.com"),
    Vendor::new("Memorex", "Memorex Products", "www.memorex.com"),
    Vendor::new("eMPIA", "eMPIA Technology", "www.empiatech.com.tw"),
    Vendor::new("Canon", "Canon", "www.canon.com"),
    Vendor::new("A4Tech", "A4tech", "www.a4tech.com"),
    Vendor::new("ALCOR", "Alcor", "www.alcor.org"),
    Vendor::new("Vimicro", "Vimicro", "www.vimicro.com"),
    Vendor::new("OTi", "Ours Technology", "www.oti.com.tw"),
    Vendor::new("BENQ", "BenQ", "www.benq.com"),
    Vendor::new("Acer", "Acer", "www.acer.com"),
    Vendor::new("QUANTUM", "Quantum", "www.quantum.com"),
    Vendor::new("Kingston", "Kingston", "www.kingston.com"),
    Vendor::new("Chicony", "Chicony", "www.chicony.com.tw"),
    Vendor::new("Genius", "Genius", "www.genius.ru"),
    /* BIOS manufacturers */
    Vendor::new("American Megatrends", "American Megatrends", "www.ami.com"),
    Vendor::new("Award", "Award Software International", "www.award-bios.com"),
    Vendor::new("Phoenix", "Phoenix Technologies", "www.phoenix.com"),
    /* x86 vendor strings */
    Vendor::new("AMDisbetter!", "Advanced Micro Devices", "www.amd.com"),
    Vendor::new("AuthenticAMD", "Advanced Micro Devices", "www.amd.com"),
    Vendor::new("CentaurHauls", "VIA (formerly Centaur Technology)", "www.via.tw"),
    Vendor::new("CyrixInstead", "Cyrix", ""),
    Vendor::new("GenuineIntel", "Intel", "www.intel.com"),
    Vendor::new("TransmetaCPU", "Transmeta", ""),
    Vendor::new("GenuineTMx86", "Transmeta", ""),
    Vendor::new("Geode by NSC", "National Semiconductor", ""),
    Vendor::new("NexGenDriven", "NexGen", ""),
    Vendor::new("RiseRiseRise", "Rise Technology", ""),
    Vendor::new("SiS SiS SiS", "Silicon Integrated Systems", ""),
    Vendor::new("UMC UMC UMC", "United Microelectronics Corporation", ""),
    Vendor::new("VIA VIA VIA", "VIA", "www.via.tw"),
    Vendor::new("Vortex86 SoC", "DMP Electronics", ""),
    /* x86 VM vendor strings */
    Vendor::new("KVMKVMKVM", "KVM", ""),
    Vendor::new("Microsoft Hv", "Microsoft Hyper-V", "www.microsoft.com"),
    Vendor::new("lrpepyh vr", "Parallels", ""),
    Vendor::new("VMwareVMware", "VMware", ""),
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
