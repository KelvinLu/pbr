//! Base58 encoding scheme.

use crate::util::number::Uint;

type U576 = Uint<576, 9>;

pub const BASE58_CHARACTERS: [char; 58] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];

/// Represents bytes that may be encoded in Base58.
pub struct Base58Encoding<'a> {
    bytes: &'a [u8]
}

impl <'a> Base58Encoding<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self { bytes: bytes }
    }
}

impl std::fmt::Display for Base58Encoding<'_> {
    /// Displays the Base58 encoding.
    ///
    /// Limited to expressions within 72 bytes (576 bits).
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        assert!(self.bytes.len() <= 72);

        let mut buffer: [u8; 72] = [0_u8; 72];

        buffer[72 - self.bytes.len()..].clone_from_slice(self.bytes);

        let encoding = Base58Encoding::encode(U576::from_be_bytes(buffer));
        let string = String::from_iter(encoding);

        write!(f, "{}", string)
    }
}

impl Base58Encoding<'_> {
    /// Convert an integer value into a Base58 string.
    ///
    /// Limited to expressions within 72 bytes (576 bits).
    pub fn encode(value: U576) -> impl Iterator<Item = char> {
        value.to_base_be(58).map(|n| BASE58_CHARACTERS[usize::try_from(n).unwrap()])
    }

    /// Convert a Base58 string into a sequence of bytes.
    ///
    /// Limited to expressions within 72 bytes (576 bits).
    pub fn decode(string: &str) -> impl Iterator<Item = u8> {
        let mut value = U576::ZERO;

        for character in string.chars() {
            let n = BASE58_CHARACTERS.iter().position(|&c| c == character).unwrap();

            value *= U576::from(58);
            value += U576::from(n);
        }

        value.to_be_bytes::<72>().into_iter().skip_while(|byte| *byte == 0_u8)
    }
}
