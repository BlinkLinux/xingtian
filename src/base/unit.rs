// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[must_use]
pub fn parse_mem_size(s: &str) -> Option<i64> {
    if s.is_empty() {
        return None;
    }

    let s = s.trim();
    let mut size: i64 = 0;
    for c in s.chars() {
        if c.is_ascii_digit() {
            size = 10 * size + i64::from(c.to_digit(10)?);
            continue;
        }

        let shift = match c {
            ' ' | '\t' | 'b' | 'B' => 0,
            'k' | 'K' => 10,
            'm' | 'M' => 20,
            'g' | 'G' => 30,
            't' | 'T' => 40,
            _ => {
                log::warn!("Invalid char in {s}");
                return None;
            }
        };
        size *= 1_i64 << shift;
    }

    Some(size)
}

#[cfg(test)]
mod tests {
    use super::parse_mem_size;

    #[test]
    fn test_parse_mem_size() {
        const PAIRS: &[(&str, Option<i64>)] = &[
            ("", None),
            ("16", Some(16)),
            ("16 \t\n", Some(16)),
            ("4k", Some(4096)),
            ("4 K", Some(4096)),
            ("4M", Some(4194304)),
            (" 4 M", Some(4194304)),
        ];
        for (key, value) in PAIRS {
            assert_eq!(parse_mem_size(key), *value);
        }
    }
}
