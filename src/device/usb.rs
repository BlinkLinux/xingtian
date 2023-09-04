// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs;
use std::path::Path;

use crate::error::Error;

#[derive(Debug, Default, Clone)]
pub struct UsbDev {
    // These attributes are read from sysfs.
    pub max_power: String,
    pub manufacturer: String,
    pub product: String,
    pub speed: String,
    pub version: String,

    // These attributes are read from sysfs uevent.
    pub major: String,
    pub minor: String,
    pub dev_name: String,
    pub dev_type: String,
    pub bus_num: String,
    pub dev_num: String,

    // These attributes are read from udev.
    pub bus: String,
    pub model: String,
    pub model_id: String,
    pub serial: String,
    pub serial_short: String,
    pub vendor: String,
    pub vendor_id: String,
    pub revision: String,

    pub usb_interfaces: String,
    pub vendor_from_database: String,
    pub model_from_database: String,
    pub path_with_usb_revision: String,
    pub path: String,
}

/// # Errors
/// Returns error if failed to parse usb devices.
pub fn scan_usb() -> Result<Vec<UsbDev>, Error> {
    const DIR: &str = "/sys/bus/usb/devices";
    let mut list = Vec::new();

    for entry in fs::read_dir(DIR).map_err(|err| Error::IoError(DIR, err))? {
        let entry = entry.map_err(|err| Error::IoError(DIR, err))?;
        let path = entry.path();
        if let Some(s) = path.to_str() {
            if !s.contains(':') {
                let dev = scan_usb_event(&path)?;
                list.push(dev);
            }
        }
    }

    Ok(list)
}

/// # Errors
/// Returns error if failed to parse usb udev info.
pub fn scan_usb_event(dir: &Path) -> Result<UsbDev, Error> {
    let mut dev = UsbDev::default();

    if let Ok(s) = fs::read_to_string(dir.join("bMaxPower")) {
        dev.max_power = s.trim().to_owned();
    } else {
        log::warn!("Failed to read usb max power file at: {dir:?}");
    }
    if let Ok(s) = fs::read_to_string(dir.join("manufacturer")) {
        dev.manufacturer = s.trim().to_owned();
    } else {
        log::warn!("Failed to read usb manufacturer file at: {dir:?}");
    }
    if let Ok(s) = fs::read_to_string(dir.join("product")) {
        dev.product = s.trim().to_owned();
    } else {
        log::warn!("Failed to read usb product file at: {dir:?}");
    }
    if let Ok(s) = fs::read_to_string(dir.join("speed")) {
        dev.speed = s.trim().to_owned();
    } else {
        log::warn!("Failed to read usb speed file at: {dir:?}");
    }
    if let Ok(s) = fs::read_to_string(dir.join("version")) {
        dev.version = s.trim().to_owned();
    } else {
        log::warn!("Failed to read usb version file at: {dir:?}");
    }

    let uevent_content =
        fs::read_to_string(dir.join("uevent")).map_err(|err| Error::IoError("usb uevent", err))?;
    for line in uevent_content.lines() {
        if !line.contains('=') {
            continue;
        }
        let mut parts = line.split('=');
        let key = if let Some(key) = parts.next() {
            key
        } else {
            log::warn!("Invalid usb uevent attr key: {line}");
            continue;
        };
        let value = if let Some(value) = parts.next() {
            value.trim()
        } else {
            log::warn!("Invalid usb uevent attr value: {line}");
            continue;
        };

        match key {
            "MAJOR" => dev.major = value.to_owned(),
            "MINOR" => dev.minor = value.to_owned(),
            "DEVNAME" => dev.dev_name = value.to_owned(),
            "DEVTYPE" => dev.dev_type = value.to_owned(),
            "BUSNUM" => dev.bus_num = value.to_owned(),
            "DEVNUM" => dev.dev_num = value.to_owned(),
            _s => (),
        }
    }

    let path = format!("/run/udev/data/c{}:{}", dev.major, dev.minor);
    let udev_content = fs::read_to_string(&path).map_err(|err| Error::IoErrorDetail(path, err))?;

    for line in udev_content.lines() {
        if !line.starts_with("E:") || !line.contains('=') {
            continue;
        }

        let mut parts = line["E:".len()..].split('=');
        let key = if let Some(key) = parts.next() {
            key
        } else {
            log::warn!("Invalid usb udev attr key: {line}");
            continue;
        };
        let value = if let Some(value) = parts.next() {
            value.trim()
        } else {
            log::warn!("Invalid usb udev attr value: {line}");
            continue;
        };

        match key {
            "ID_BUS" => dev.bus = value.to_owned(),
            "ID_MODEL" => dev.model = value.to_owned(),
            "ID_MODEL_ID" => dev.model_id = value.to_owned(),
            "ID_SERIAL" => dev.serial = value.to_owned(),
            "ID_SERIAL_SHORT" => dev.serial_short = value.to_owned(),
            "ID_VENDOR" => dev.vendor = value.to_owned(),
            "ID_VENDOR_ID" => dev.vendor_id = value.to_owned(),
            "ID_REVISION" => dev.revision = value.to_owned(),
            "ID_USB_INTERFACES" => dev.usb_interfaces = value.to_owned(),
            "ID_VENDOR_FROM_DATABASE" => dev.vendor_from_database = value.to_owned(),
            "ID_MODEL_FROM_DATABASE" => dev.model_from_database = value.to_owned(),
            "ID_PATH_WITH_USB_REVISION" => dev.path_with_usb_revision = value.to_owned(),
            "ID_PATH" => dev.path = value.to_owned(),
            _s => (),
        }
    }

    Ok(dev)
}

#[cfg(test)]
mod tests {
    use super::scan_usb;

    #[test]
    fn test_scan_usb() {
        let list = scan_usb();
        assert!(list.is_ok());
        assert!(list.unwrap().len() > 2);
    }
}
