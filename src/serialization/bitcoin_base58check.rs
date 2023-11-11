//! Base58Check encoding scheme (Bitcoin).

use std::iter::repeat;

use crate::serialization::base58::Base58Encoding;
use crate::serialization::bytes::count_leading_zero_bytes;
use crate::crypto::digest::hash_256;
use crate::util::number::Uint;

type U576 = Uint<576, 9>;

/// Represents bytes that may be encoded in Base58Check (a Bitcoin-specific variant that may include
/// prefix and suffix metadata, and contains a checksum).
pub struct Base58CheckBitcoinEncoding<'a> {
    bytes: &'a [u8]
}

impl <'a> Base58CheckBitcoinEncoding<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self { bytes: bytes }
    }
}

/// Bitcoin prefix bytes.
///
/// Prepended to payload data to indicate metadata, prior to applying the Base58 encoding.
///
/// This determines the starting character pattern of a given encoding, to easily distinguish
/// between different kinds of items.
///
/// # Mainnet
///
/// - `0x00`: mainnet p2pkh address ("`1...`")
/// - `0x05`: mainnet p2sh address ("`3...`")
///
/// - `0x80`: mainnet wallet import format private key ("`{K,L,5}...`")
///
/// - `0x0488ade4`: mainnet extended private key ("`xprv...`")
/// - `0x0488b21e`: mainnet extended public key ("`xpub...`")
///
/// # Testnet
///
/// - `0x6f`: testnet p2pkh address ("`{m,n}...`")
/// - `0xc4`: testnet p2sh address ("`2...`")
///
/// - `0xef`: testnet wallet import format private key ("`{c,9}...`")
///
/// - `0x04358394`: testnet extended private key ("`tprv...`")
/// - `0x043578cf`: testnet extended public key ("`tpub...`")
pub enum BitcoinEncodingPrefix {
    // Mainnet
    MainnetP2pkhAddress,
    MainnetP2shAddress,

    MainnetWifPrivateKey,

    MainnetExtendedPrivateKey,
    MainnetExtendedPublicKey,

    // Testnet
    TestnetP2pkhAddress,
    TestnetP2shAddress,

    TestnetWifPrivateKey,

    TestnetExtendedPrivateKey,
    TestnetExtendedPublicKey,
}

/// Bitcoin suffix bytes.
///
/// Appended to payload data to indicate metadata, prior to applying the Base58 encoding.
///
/// - `0x01`: used within wallet import format to communicate that the related public key is
///   expressed as a compressed point
pub enum BitcoinEncodingSuffix {
    WifWithCompressedPoint,
}

impl BitcoinEncodingPrefix {
    pub const fn bytes(self) -> &'static [u8] {
        match self {
            // Mainnet
            Self::MainnetP2pkhAddress => &[0x00],
            Self::MainnetP2shAddress => &[0x05],

            Self::MainnetWifPrivateKey => &[0x80],

            Self::MainnetExtendedPrivateKey => &[0x04, 0x88, 0xad, 0xe4],
            Self::MainnetExtendedPublicKey => &[0x04, 0x88, 0xb2, 0x1e],

            // Testnet
            Self::TestnetP2pkhAddress => &[0x6f],
            Self::TestnetP2shAddress => &[0xc4],

            Self::TestnetWifPrivateKey => &[0xef],

            Self::TestnetExtendedPrivateKey => &[0x04, 0x35, 0x83, 0x94],
            Self::TestnetExtendedPublicKey => &[0x04, 0x35, 0x78, 0xcf],
        }
    }
}

impl BitcoinEncodingSuffix {
    pub fn bytes(self) -> &'static [u8] {
        match self {
            Self::WifWithCompressedPoint => &[0x01],
        }
    }
}

impl std::fmt::Display for Base58CheckBitcoinEncoding<'_> {
    /// Displays the Base58Check encoding.
    ///
    /// Limited to expressions within 72 bytes (576 bits), where four bytes are reserved for the
    /// checksum, and the rest are used for payload data, prefix bytes, and suffix bytes.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        assert!(self.bytes.len() <= 68);

        let mut buffer : [u8; 72] = [0; 72];

        let checksum = &hash_256(self.bytes)[0..4];

        buffer[68..72].clone_from_slice(checksum);

        let leading_zeroes_payload = count_leading_zero_bytes(self.bytes);
        let offset = 68 - self.bytes.len();

        buffer[offset..68].clone_from_slice(&self.bytes);

        let encoding = Base58Encoding::encode(U576::from_be_bytes(buffer));
        let string = String::from_iter(repeat('1').take(leading_zeroes_payload.into()).chain(encoding));

        write!(f, "{}", string)
    }
}

#[derive(Debug)]
pub struct Base58CheckError;

impl Base58CheckBitcoinEncoding<'_> {
    /// Decodes a Base58Check string into an expected number of bytes.
    ///
    /// Verifies the checksum, and returns an error if it could not be verified.
    ///
    /// Limited to expressions within 72 bytes (576 bits).
    pub fn decode(n: usize, string: &str) -> Result<Vec<u8>, Base58CheckError> {
        let mut bytes: Vec<u8> = Base58Encoding::decode(string).collect();
        let length = bytes.len();

        assert!(length >= 4);
        assert!(length <= n + 4);

        let length = length - 4;

        for _ in 0..(n - length) { bytes.insert(0, 0_u8) }

        let data = &bytes[0..n];

        if &bytes[n..(n + 4)] != &hash_256(data)[0..4] {
            return Err(Base58CheckError);
        }

        Ok(data.to_vec())
    }
}
