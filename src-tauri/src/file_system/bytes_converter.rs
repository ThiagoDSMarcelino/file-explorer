#![allow(dead_code)]

pub(super) fn bytes_to_gigabytes(bytes: u64) -> f64 {
    bytes as f64 / 1_073_741_824.0
}

pub(super) fn bytes_to_megabytes(bytes: u64) -> f64 {
    bytes as f64 / 1_048_576.0
}

pub(super) fn bytes_to_kilobytes(bytes: u64) -> f64 {
    bytes as f64 / 1024.0
}
