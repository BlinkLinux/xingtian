// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IoError to read file: `{0}`, reason: `{1}`")]
    IoError(&'static str, io::Error),

    #[error("Failed to parse `{0}`, reason: `{1}`")]
    ParseFile(&'static str, &'static str),
}
