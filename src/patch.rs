//! Patches Paradox game executables to bypass ironman checksum validation.
//!
//! Searches for x86-64 patterns containing `test eax, eax` (85 C0) and replaces with `xor eax, eax`
//! (31 C0), forcing the checksum validation to always return success.

use color_eyre::{Result, eyre, eyre::WrapErr};
use hex_literal::hex;
use std::{fs, path::Path};
use tracing::{debug, trace};

const SEARCH_LIMIT: usize = 14;

const START_PATTERNS: &[[u8; 3]] =
    &[hex!("48 8B 12"), hex!("48 8D 0D"), hex!("48 8B D0"), hex!("48 8D 0D")];

const END_PATTERNS: &[[u8; 6]] = &[hex!("85 C0 0F 94 C3 E8"), hex!("85 C0 0F 94 C1 88")];

const REPLACEMENT: &[u8] = &hex!("31 C0 0F 94 C3 E8");
const REPLACEMENT_EU5: &[u8] = &hex!("31 C0 0F 94 C1 88");

pub fn apply_patch(path: &Path) -> Result<()> {
    let mut bytes = fs::read(path).wrap_err("failed to read file")?;

    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    modify_bytes(filename, &mut bytes)?;
    backup_file(path)?;

    fs::write(path, &bytes).wrap_err("failed to write patched file")?;

    Ok(())
}

fn modify_bytes(filename: &str, bytes: &mut [u8]) -> Result<()> {
    let mut matches_count: usize = 0;
    let replacement = get_replacement(filename);
    let start_len = START_PATTERNS[0].len();
    let end_len = END_PATTERNS[0].len();

    for i in 0..bytes.len().saturating_sub(start_len) {
        if let Some(start_slice) = bytes.get(i..i + start_len)
            && is_start_candidate(start_slice)
        {
            let search_end =
                (i + start_len + SEARCH_LIMIT).min(bytes.len().saturating_sub(end_len));

            for j in (i + start_len)..=search_end {
                if let Some(end_slice) = bytes.get(j..j + end_len)
                    && is_end_candidate(end_slice)
                {
                    matches_count += 1;
                    trace!("found match #{}", matches_count);
                    bytes[j..j + replacement.len()].copy_from_slice(replacement);
                    break;
                }
            }
        }
    }

    if matches_count == 0 {
        eyre::bail!(
            "cannot detect bytes pattern to patch. Most likely patcher is outdated \
             due to game updates or patch has already been applied"
        );
    }

    debug!("found {} match(es)", matches_count);

    Ok(())
}

fn is_start_candidate(bytes: &[u8]) -> bool {
    START_PATTERNS.iter().any(|&pattern| bytes == pattern)
}

fn is_end_candidate(bytes: &[u8]) -> bool {
    END_PATTERNS.iter().any(|&pattern| bytes == pattern)
}

fn get_replacement(filename: &str) -> &'static [u8] {
    if filename == "eu5.exe" { REPLACEMENT_EU5 } else { REPLACEMENT }
}

fn backup_file(original: &Path) -> Result<()> {
    let data = fs::read(original).wrap_err("failed to read file for backup")?;
    let backup_path = original.with_extension(
        original.extension().and_then(|e| e.to_str()).unwrap_or("exe").to_owned() + ".backup",
    );
    fs::write(&backup_path, &data).wrap_err("failed to write backup file")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_start_candidate() {
        for pattern in START_PATTERNS {
            assert!(is_start_candidate(pattern));
        }
        assert!(!is_start_candidate(&[0x00, 0x00, 0x00]));
    }

    #[test]
    fn test_is_end_candidate() {
        for pattern in END_PATTERNS {
            assert!(is_end_candidate(pattern));
        }
        assert!(!is_end_candidate(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]));
    }

    #[test]
    fn test_get_replacement() {
        assert_eq!(get_replacement("eu5.exe"), REPLACEMENT_EU5);
        assert_eq!(get_replacement("eu4.exe"), REPLACEMENT);
        assert_eq!(get_replacement("hoi4.exe"), REPLACEMENT);
    }
}
