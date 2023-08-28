// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs;
use std::time::Duration;

use crate::error::Error;

/// # Errors
/// Returns error if failed to read uptime file or failed to parse file content.
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub fn get_uptime() -> Result<Duration, Error> {
    const FILE: &str = "/proc/uptime";
    let content = fs::read_to_string(FILE).map_err(|err| Error::IoError(FILE, err))?;

    let uptime_str = content
        .split(' ')
        .next()
        .ok_or_else(|| Error::ParseFile(FILE, "Failed to split content"))?;
    let seconds: f64 = uptime_str
        .parse()
        .map_err(|_err| Error::ParseFile(FILE, "Failed to convert uptime to integer"))?;

    let millis: u64 = (seconds * 1000.0) as u64;
    let duration = Duration::from_millis(millis);
    Ok(duration)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::get_uptime;

    #[test]
    fn test_get_uptime() {
        let uptime = get_uptime();
        assert!(uptime.is_ok());
        assert!(uptime.unwrap() > Duration::from_secs(1));
    }
}
