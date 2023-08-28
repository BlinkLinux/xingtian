// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::env;
use std::ffi::OsString;

pub struct EnvItem {
    pub key: OsString,
    pub value: OsString,
}

impl From<(OsString, OsString)> for EnvItem {
    fn from((key, value): (OsString, OsString)) -> Self {
        Self { key, value }
    }
}

pub fn get_environment() -> Vec<EnvItem> {
    env::vars_os().map(EnvItem::from).collect()
}

#[cfg(test)]
mod tests {
    use super::get_environment;

    #[test]
    fn test_get_environment() {
        let env_list = get_environment();
        assert!(!env_list.is_empty());
    }
}
