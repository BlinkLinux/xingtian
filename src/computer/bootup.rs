// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::error::Error;

#[derive(Debug, Default, Clone)]
pub struct Bootup {
    pub username: String,
    pub session_tty: String,
    pub kernel: String,
    pub start_time: String,
    pub end_time: String,
}

/// # Panics
/// got panic if failed to parse wtmp
///
/// # Errors
/// Returns error if failed to parse bootup time.
pub fn get_list() -> Result<Vec<Bootup>, Error> {
    const FILE: &str = "/home/shaohua/Desktop/wtmp";
    let entries = utmp_rs::parse_from_path(FILE);
    for entry in entries.unwrap() {
        println!("entry: {entry:#?}");
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::get_list;

    #[test]
    fn test_get_list() {
        let list = get_list();
        assert!(list.is_ok());
        let list = list.unwrap();
        assert!(!list.is_empty());
        assert!(list.is_empty());
        //ASSERT_LT(list.at(0).start_time, QDateTime::currentDateTime());
    }
}
