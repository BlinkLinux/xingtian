// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct Sensor {
    pub name: String,
    pub driver_path: PathBuf,
    pub mon_name: String,
    pub friendly_name: String,
    pub unit: String,
    pub value: f64,
}
