//! Bitcoin WIF (wallet import format).

use crate::serialization::bitcoin_base58check::Base58CheckBitcoinEncoding;
use crate::serialization::bitcoin_base58check::BitcoinEncodingPrefix;
use crate::serialization::bitcoin_base58check::BitcoinEncodingSuffix;
use crate::bitcoin::network::BitcoinNetworkType;
use crate::util::number::U256;

/// Represents a Bitcoin WIF (wallet import format) encoding.
///
/// Stores one or two bytes to denote metadata, as well the 32 byte representation of the (private
/// key) integer.
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct BitcoinWif {
    compressed: bool,

    bytes: [u8; 34]
}

impl BitcoinWif {
    /// Create a Bitcoin WIF encoding from a (private key) integer `secret_e`.
    pub fn for_secret_e(network: BitcoinNetworkType, secret_e: U256, compressed_point: bool) -> Self {
        let mut bytes = [0_u8; 34];

        let prefix_bytes =
            match network {
                BitcoinNetworkType::Mainnet => BitcoinEncodingPrefix::bytes(BitcoinEncodingPrefix::MainnetWifPrivateKey),
                BitcoinNetworkType::Testnet => BitcoinEncodingPrefix::bytes(BitcoinEncodingPrefix::TestnetWifPrivateKey),
            };

        bytes[0..=0].clone_from_slice(prefix_bytes);
        bytes[1..=32].clone_from_slice(&secret_e.to_be_bytes::<32>());

        if compressed_point {
            bytes[33..=33].clone_from_slice(&BitcoinEncodingSuffix::bytes(BitcoinEncodingSuffix::WifWithCompressedPoint));
        }

        Self {
            compressed: compressed_point,
            bytes: bytes,
        }
    }
}

impl std::fmt::Display for BitcoinWif {
    /// Displays the Bitcoin WIF encoding, formatted with Base58Check.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let bytes: &[u8]=
            if self.compressed {
                &self.bytes
            } else {
                &self.bytes[0..=32]
            };

        let encoding = Base58CheckBitcoinEncoding::from_bytes(bytes);

        write!(f, "{}", encoding.to_string())
    }
}
