//! Variable integers.

/// Read from a series of bytes, and return a 64-bit variable integer, as well as the bytes read.
///
/// Variable bytes after the leading byte are interpreted as little-endian.
///
/// If the required amount of bytes could not be read, no value is returned instead.
pub fn read_varint_u64<'a>(mut bytes: impl Iterator<Item = &'a u8>) -> Option<(u64, usize)> {
    let mut buffer: [u8; 8] = [0_u8; 8];
    let mut i: usize = 0;
    let append_byte = |n: &u8| {
        buffer[i] = *n;
        i += 1;
    };

    match bytes.next() {
        Some(leading_byte) if *leading_byte == 0xff_u8 => {
            bytes.take(8).for_each(append_byte);

            if i != 8 { return None }

            Some((u64::from_le_bytes(buffer), 8))
        },
        Some(leading_byte) if *leading_byte == 0xfe_u8 => {
            bytes.take(4).for_each(append_byte);

            if i != 4 { return None }

            Some((u64::from_le_bytes(buffer), 4))
        },
        Some(leading_byte) if *leading_byte == 0xfd_u8 => {
            bytes.take(2).for_each(append_byte);

            if i != 2 { return None }

            Some((u64::from_le_bytes(buffer), 2))
        },
        Some(leading_byte) => {
            Some((u64::from(*leading_byte), 1))
        },
        _ => None
    }
}

/// Produce a series of bytes that represent a little-endian variable integer, and also return the
/// amount of bytes to be considered.
pub fn varint_u64(value: u64) -> ([u8; 9], usize) {
    let mut buffer: [u8; 9] = [0_u8; 9];

    match value {
        0x00000000ffffffff..=0xffffffffffffffff => {
            // (2^32) - (2^64 - 1)
            buffer[0] = 0xff_u8;
            buffer[1..=8].clone_from_slice(&value.to_le_bytes());

            (buffer, 9)
        },
        0x000000000000ffff..=0x00000000fffffffe => {
            // (2^16) - (2^32 - 1)
            buffer[0] = 0xfe_u8;
            buffer[1..=4].clone_from_slice(&u32::try_from(value).unwrap().to_le_bytes());

            (buffer, 5)
        },
        0x00000000000000fd..=0x000000000000fffe => {
            // 232 - (2^16 - 1)
            buffer[0] = 0xfd_u8;
            buffer[1..=2].clone_from_slice(&u16::try_from(value).unwrap().to_le_bytes());

            (buffer, 3)
        },
        0x0000000000000000..=0x00000000000000fc => {
            // 0 - 252
            buffer[0] = u8::try_from(value).unwrap();

            (buffer, 1)
        },
    }
}
