// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs;
use std::path::Path;

use crate::error::Error;

#[derive(Debug, Default, Clone)]
pub struct PowerSupply {
    pub name: String,
    pub manufacturer: String,
    pub model_name: String,
    pub serial_number: String,
    pub technology: Technology,
    pub type_: Type,
    pub status: Status,
    pub present: bool,
    pub cycle_count: i32,

    pub voltage_min_design: i32,
    pub voltage_now: i32,
    pub current_now: i32,
    pub charge_full_design: i32,
    pub charge_full: i32,
    pub charge_now: i32,
    pub capacity: u8,
    pub capacity_level: CapacityLevel,

    pub alarm: u8,
    pub charge_control_end_threshold: u8,
    pub charge_control_start_threshold: u8,
    // TODO(Shaohua): Add device path
    // TODO(Shaohua): Add hwmon path
}

/// Power supply type.
///
/// From `power_supply_sysfs.c`
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Type {
    Unknown,
    Battery,
    UPS,
    Mains,
    USB,
    USB_DCP,
    USB_CDP,
    USB_ACA,
    USB_C,
    USB_PD,
    USB_PD_DRP,
    BrickID,
    Wireless,
}

impl Default for Type {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<&str> for Type {
    fn from(s: &str) -> Self {
        match s {
            "Unknown" => Self::Unknown,
            "Battery" => Self::Battery,
            "UPS" => Self::UPS,
            "Mains" => Self::Mains,
            "USB" => Self::USB,
            "USB_DCP" => Self::USB_DCP,
            "USB_CDP" => Self::USB_CDP,
            "USB_ACA" => Self::USB_ACA,
            "USB_C" => Self::USB_C,
            "USB_PD" => Self::USB_PD,
            "USB_PD_DRP" => Self::USB_PD_DRP,
            "BrickID" => Self::BrickID,
            "Wireless" => Self::Wireless,
            s => {
                log::warn!("Unknown type of power supply: {s}");
                Self::Unknown
            }
        }
    }
}

/// Power supply status.
///
/// From `power_supply_sysfs.c`
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Status {
    Unknown,
    Charging,
    Discharging,
    NotCharging,
    Full,
}

impl Default for Status {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        match s {
            "Unknown" => Self::Unknown,
            "Charging" => Self::Charging,
            "Discharging" => Self::Discharging,
            "Not charging" => Self::NotCharging,
            "Full" => Self::Full,
            s => {
                log::warn!("Unknown status of power supply: {s}");
                Self::Unknown
            }
        }
    }
}

/// Power supply technology.
///
/// From `power_supply_sysfs.c`
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Technology {
    Unknown,
    NiMH,
    LiIon,
    LiPoly,
    LiFe,
    NiCd,
    LiMn,
}

impl Default for Technology {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<&str> for Technology {
    fn from(s: &str) -> Self {
        match s {
            "Unknown" => Self::Unknown,
            "NiMH" => Self::NiMH,
            "Li-ion" => Self::LiIon,
            "Li-poly" => Self::LiPoly,
            "LiFe" => Self::LiFe,
            "NiCd" => Self::NiCd,
            "LiMn" => Self::LiMn,
            s => {
                log::warn!("Unknown technology of power supply: {s}");
                Self::Unknown
            }
        }
    }
}

/// Capacity level of power supply.
///
/// From `power_supply_sysfs.c`
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CapacityLevel {
    Unknown,
    Critical,
    Low,
    Normal,
    High,
    Full,
}

impl Default for CapacityLevel {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<&str> for CapacityLevel {
    fn from(s: &str) -> Self {
        match s {
            "Unknown" => Self::Unknown,
            "Critical" => Self::Critical,
            "Low" => Self::Low,
            "Normal" => Self::Normal,
            "High" => Self::High,
            "Full" => Self::Full,
            s => {
                log::warn!("Unknown capacity level of power supply: {s}");
                Self::Unknown
            }
        }
    }
}

/// # Errors
/// Returns error if failed to read power supply file.
pub fn get_list() -> Result<Vec<PowerSupply>, Error> {
    const DIR: &str = "/sys/class/power_supply";
    let mut list = Vec::new();

    for entry in fs::read_dir(DIR).map_err(|err| Error::IoError(DIR, err))? {
        let entry = entry.map_err(|err| Error::IoError(DIR, err))?;
        let path = entry.path();
        if path.is_dir() {
            let filename = path.file_name().unwrap_or_default();
            if filename.is_ascii() && filename.to_str().unwrap_or_default().starts_with("BAT") {
                let battery = read_detail(&path)?;
                list.push(battery);
            }
        }
    }

    Ok(list)
}

/// # Errors
/// Returns error if failed to parse power supply directory.
pub fn read_detail(dir: &Path) -> Result<PowerSupply, Error> {
    const FILE: &str = "power_supply/uevent";

    let uevent = fs::read_to_string(dir.join("uevent")).map_err(|err| Error::IoError(FILE, err))?;

    let mut ps = PowerSupply::default();

    for line in uevent.lines() {
        if !line.starts_with("POWER_SUPPLY_") {
            log::warn!("Invalid power supply attr: {line}");
            continue;
        }

        let mut parts = line["POWER_SUPPLY_".len()..].split('=');
        let key = if let Some(key) = parts.next() {
            key
        } else {
            log::warn!("Invalid power supply attr key: {line}");
            continue;
        };
        let value = if let Some(value) = parts.next() {
            value.trim()
        } else {
            log::warn!("Invalid power supply attr value: {line}");
            continue;
        };

        match key {
            "NAME" => ps.name = value.to_owned(),
            "TYPE" => ps.type_ = value.into(),
            "STATUS" => ps.status = value.into(),
            "PRESENT" => ps.present = value == "1",
            "TECHNOLOGY" => ps.technology = value.into(),
            "CYCLE_COUNT" => {
                ps.cycle_count = value
                    .parse()
                    .map_err(|_err| Error::ParseFile(FILE, "Failed to parse cycle count"))?;
            }
            "VOLTAGE_MIN_DESIGN" => {
                ps.voltage_min_design = value
                    .parse()
                    .map_err(|_err| Error::ParseFile(FILE, "Failed to parse voltage_min_design"))?;
            }
            "VOLTAGE_NOW" => {
                ps.voltage_now = value
                    .parse()
                    .map_err(|_err| Error::ParseFile(FILE, "Failed to parse voltage_now"))?;
            }
            "CURRENT_NOW" => {
                ps.current_now = value
                    .parse()
                    .map_err(|_err| Error::ParseFile(FILE, "Failed to parse current_now"))?;
            }
            "CHARGE_FULL_DESIGN" => {
                ps.charge_full_design = value
                    .parse()
                    .map_err(|_err| Error::ParseFile(FILE, "Failed to parse charge_full_design"))?;
            }
            "CHARGE_FULL" => {
                ps.charge_full = value
                    .parse()
                    .map_err(|_err| Error::ParseFile(FILE, "Failed to parse charge_full"))?;
            }
            "CHARGE_NOW" => {
                ps.charge_now = value
                    .parse()
                    .map_err(|_err| Error::ParseFile(FILE, "Failed to parse charge_now"))?;
            }
            "CAPACITY" => {
                ps.capacity = value
                    .parse()
                    .map_err(|_err| Error::ParseFile(FILE, "Failed to parse capacity"))?;
            }
            "CAPACITY_LEVEL" => ps.capacity_level = value.into(),
            "MODEL_NAME" => ps.model_name = value.to_owned(),
            "MANUFACTURER" => ps.manufacturer = value.to_owned(),
            "SERIAL_NUMBER" => ps.serial_number = value.to_owned(),
            _key => log::warn!("Invalid power supply attr key: {line}"),
        }
    }

    if let Ok(alarm_str) = fs::read_to_string(dir.join("alarm")) {
        ps.alarm = alarm_str
            .trim()
            .parse()
            .map_err(|_err| Error::ParseFile("power_supply/alarm", ""))?;
    } else {
        log::warn!("Failed to read power_supply/alarm file");
    }

    if let Ok(s) = fs::read_to_string(dir.join("charge_control_end_threshold")) {
        ps.charge_control_end_threshold = s
            .trim()
            .parse()
            .map_err(|_err| Error::ParseFile("power_supply/charge_control_end_threshold", ""))?;
    } else {
        log::warn!("Failed to read power_supply/charge_control_end_threshold file");
    }

    if let Ok(s) = fs::read_to_string(dir.join("charge_control_start_threshold")) {
        ps.charge_control_start_threshold = s
            .trim()
            .parse()
            .map_err(|_err| Error::ParseFile("power_supply/charge_control_start_threshold", ""))?;
    } else {
        log::warn!("Failed to read power_supply/charge_control_start_threshold file");
    }

    Ok(ps)
}

#[cfg(test)]
mod tests {
    use super::get_list;

    #[test]
    fn test_get_list() {
        let list = get_list();
        assert!(list.is_ok());
    }
}
