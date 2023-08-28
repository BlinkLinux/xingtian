// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Clone)]
pub struct Language {
    pub locale: String,
    pub title: String,
    pub source: String,
    pub address: String,
    pub email: String,
    pub language: String,
    pub territory: String,
    pub revision: String,
    pub date: String,
    pub code_set: String,
}
