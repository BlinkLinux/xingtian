// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs;

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub memory_bytes: usize,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ModuleDetail {
    pub name: String,
    pub description: String,
    pub version_magic: String,

    pub path: String,
    pub memory_bytes: usize,

    pub dependencies: Vec<String>,

    pub author: String,
    pub license: String,

    pub signer: String,
    pub sig_key: String,
}

/// # Errors
/// Returns error if failed to read modules file or failed to parse content.
pub fn get_list() -> Result<Vec<Module>, Error> {
    const FILE: &str = "/proc/modules";
    let content = fs::read_to_string(FILE).map_err(|err| Error::IoError(FILE, err))?;
    let mut modules = Vec::new();

    for line in content.lines() {
        let mut iter = line.split_ascii_whitespace();
        let name = iter
            .next()
            .ok_or_else(|| Error::ParseFile(FILE, "Failed to parse module name"))?
            .to_string();
        let memory_bytes: usize = iter
            .next()
            .ok_or_else(|| Error::ParseFile(FILE, "Failed to parse memory bytes"))?
            .parse()
            .map_err(|_err| Error::ParseFile(FILE, "Failed to parse memory bytes as integer"))?;
        let _count: i32 = iter
            .next()
            .ok_or_else(|| Error::ParseFile(FILE, "Failed to parse count"))?
            .parse()
            .map_err(|_err| Error::ParseFile(FILE, "Failed to parse count as integer"))?;

        let dependencies: Vec<String> = match iter
            .next()
            .ok_or_else(|| Error::ParseFile(FILE, "Failed to parse dependencies"))?
        {
            "-" => Vec::new(),
            deps => deps
                .split(',')
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
                .collect(),
        };

        modules.push(Module {
            name,
            memory_bytes,
            dependencies,
        });
    }

    Ok(modules)
}

/// # Errors
/// Returns error if module not found or failed to read ko file.
pub fn get_detail(_name: &str) -> Result<ModuleDetail, Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{get_detail, get_list};

    #[test]
    fn test_get_list() {
        let module_list = get_list();
        assert!(module_list.is_ok());
        assert!(module_list.unwrap().len() > 10);
    }

    #[test]
    fn test_get_detail() {
        let name = "stp";
        let module_detail = get_detail(name);
        assert!(module_detail.is_ok());
        assert_eq!(module_detail.unwrap().dependencies, ["llc"]);
    }
}
