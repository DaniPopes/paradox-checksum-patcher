use anyhow::{Context, Result};
use std::fs;
use tracing::{debug, trace};

const LIMIT: usize = 14;

// Start patterns.
const START1: &[u8] = &[0x48, 0x8B, 0x12];
const START2: &[u8] = &[0x48, 0x8D, 0x0D];
const START3: &[u8] = &[0x48, 0x8B, 0xD0];
const START4: &[u8] = &[0x48, 0x8D, 0x0D];
const START_LENGTH: usize = 3;

// End patterns.
const END: &[u8] = &[0x85, 0xC0, 0x0F, 0x94, 0xC3, 0xE8];
const END_EU5: &[u8] = &[0x85, 0xC0, 0x0F, 0x94, 0xC1, 0x88];
const END_LENGTH: usize = 6;

// Replacement patterns.
const REPLACEMENT: &[u8] = &[0x31, 0xC0, 0x0F, 0x94, 0xC3, 0xE8];
const REPLACEMENT_EU5: &[u8] = &[0x31, 0xC0, 0x0F, 0x94, 0xC1, 0x88];
const REPLACEMENT_LENGTH: usize = 6;

#[inline]
fn is_start_candidate(bytes: &[u8]) -> bool {
    bytes == START1 || bytes == START2 || bytes == START3 || bytes == START4
}

#[inline]
fn is_end_candidate(bytes: &[u8]) -> bool {
    bytes == END || bytes == END_EU5
}

#[inline]
fn get_replacement(filename: &str) -> &'static [u8] {
    if filename == "eu5.exe" {
        REPLACEMENT_EU5
    } else {
        REPLACEMENT
    }
}

pub fn apply_patch(filename: &str) -> Result<()> {
    let mut bytes = fs::read(filename).context("failed to read file")?;

    modify_bytes(filename, &mut bytes)?;
    backup_file(filename)?;

    fs::write(filename, &bytes).context("failed to write patched file")?;

    Ok(())
}

fn modify_bytes(filename: &str, bytes: &mut [u8]) -> Result<()> {
    let mut matches_count = 0;
    let bytes_length = bytes.len();
    let replacement = get_replacement(filename);

    let mut i = 0;
    while i <= bytes_length.saturating_sub(START_LENGTH) {
        if is_start_candidate(&bytes[i..i + START_LENGTH]) {
            let search_end = (i + START_LENGTH + LIMIT).min(bytes_length - END_LENGTH);

            for j in (i + START_LENGTH)..=search_end {
                if is_end_candidate(&bytes[j..j + END_LENGTH]) {
                    matches_count += 1;
                    trace!("found match #{}", matches_count);
                    bytes[j..j + REPLACEMENT_LENGTH].copy_from_slice(replacement);
                    break;
                }
            }
        }
        i += 1;
    }

    if matches_count == 0 {
        anyhow::bail!(
            "cannot detect bytes pattern to patch. Most likely patcher is outdated \
             due to game updates or patch has already been applied"
        );
    }

    debug!("found {} match(es)", matches_count);

    Ok(())
}

fn backup_file(original: &str) -> Result<()> {
    let data = fs::read(original).context("failed to read file for backup")?;
    let backup_path = format!("{}.backup", original);
    fs::write(&backup_path, &data).context("failed to write backup file")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_start_candidate() {
        assert!(is_start_candidate(START1));
        assert!(is_start_candidate(START2));
        assert!(is_start_candidate(START3));
        assert!(is_start_candidate(START4));
        assert!(!is_start_candidate(&[0x00, 0x00, 0x00]));
    }

    #[test]
    fn test_is_end_candidate() {
        assert!(is_end_candidate(END));
        assert!(is_end_candidate(END_EU5));
        assert!(!is_end_candidate(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]));
    }

    #[test]
    fn test_get_replacement() {
        assert_eq!(get_replacement("eu5.exe"), REPLACEMENT_EU5);
        assert_eq!(get_replacement("eu4.exe"), REPLACEMENT);
        assert_eq!(get_replacement("hoi4.exe"), REPLACEMENT);
    }
}
