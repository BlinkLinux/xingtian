// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Group {
    pub name: String,
    pub gid: i32,
    pub members: Vec<String>,
}

/// # Errors
///
/// # Panics
pub fn get_group_list() -> Result<Vec<Group>, Error> {
    todo!()
}
