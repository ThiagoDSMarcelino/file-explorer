#![allow(dead_code)]

/// Converts bytes to gigabytes.
///
/// # Arguments
///
/// * `bytes` - The number of bytes to convert.
///
/// # Returns
///
/// The equivalent value in gigabytes.
pub(super) fn bytes_to_gigabytes(bytes: u64) -> f64 {
    bytes as f64 / 1_073_741_824.0
}

/// Converts bytes to megabytes.
///
/// # Arguments
///
/// * `bytes` - The number of bytes to convert.
///
/// # Returns
///
/// The equivalent value in megabytes.
pub(super) fn bytes_to_megabytes(bytes: u64) -> f64 {
    bytes as f64 / 1_048_576.0
}

/// Converts bytes to kilobytes.
///
/// # Arguments
///
/// * `bytes` - The number of bytes to convert.
///
/// # Returns
///
/// The equivalent value in kilobytes.
pub(super) fn bytes_to_kilobytes(bytes: u64) -> f64 {
    bytes as f64 / 1024.0
}
