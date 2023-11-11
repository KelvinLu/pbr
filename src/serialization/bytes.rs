//! Utility functions for interacting with bytes.

pub fn count_leading_zero_bytes(bytes: &[u8]) -> u8 {
    let mut n = 0;
    let iterator = bytes.iter();

    for byte in iterator {
        if *byte == 0_u8 {
            n += 1;
        } else {
            return n;
        }
    }

    n
}
