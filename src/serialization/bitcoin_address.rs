//! Bitcoin addresses.

use crate::serialization::bitcoin_base58check::Base58CheckBitcoinEncoding;
use crate::serialization::bitcoin_base58check::BitcoinEncodingPrefix;
use crate::serialization::point::CompressedPointSecFormatBytes;
use crate::serialization::point::UncompressedPointSecFormatBytes;
use crate::bitcoin::network::BitcoinNetworkType;
use crate::bitcoin::script::ScriptBytes;
use crate::bitcoin::script_types::BitcoinTransactionType;
use crate::crypto::digest::hash_160;
use crate::util::byte_string::ByteSlice;

/// Represents a Bitcoin address.
///
/// Stores a one byte prefix to denote address type/metadata, as well a 20 byte `HASH_160` digest
/// of the underlying material (i.e.; public key hash, script hash).
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct BitcoinAddress {
    bytes: [u8; 21]
}

impl std::fmt::Display for BitcoinAddress {
    /// Displays the Bitcoin address, formatted with Base58Check.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let encoding = Base58CheckBitcoinEncoding::from_bytes(&self.bytes);

        write!(f, "{}", encoding.to_string())
    }
}

const MAINNET_P2PKH: u8 = BitcoinEncodingPrefix::MainnetP2pkhAddress.bytes()[0];
const MAINNET_P2SH: u8 = BitcoinEncodingPrefix::MainnetP2shAddress.bytes()[0];
const TESTNET_P2PKH: u8 = BitcoinEncodingPrefix::TestnetP2pkhAddress.bytes()[0];
const TESTNET_P2SH: u8 = BitcoinEncodingPrefix::TestnetP2shAddress.bytes()[0];

impl BitcoinAddress {
    pub fn base58_encoding_type(network: BitcoinNetworkType, address_type: BitcoinTransactionType) -> BitcoinEncodingPrefix {
        match (network, address_type) {
            (BitcoinNetworkType::Mainnet, BitcoinTransactionType::P2pkh) => BitcoinEncodingPrefix::MainnetP2pkhAddress,
            (BitcoinNetworkType::Mainnet, BitcoinTransactionType::P2sh) => BitcoinEncodingPrefix::MainnetP2shAddress,

            (BitcoinNetworkType::Testnet, BitcoinTransactionType::P2pkh) => BitcoinEncodingPrefix::TestnetP2pkhAddress,
            (BitcoinNetworkType::Testnet, BitcoinTransactionType::P2sh) => BitcoinEncodingPrefix::TestnetP2shAddress,

            _ => panic!("unknown network and/or transaction type")
        }
    }

    /// Returns the network type.
    pub fn network(&self) -> BitcoinNetworkType {
        match self.bytes[0] {
            | MAINNET_P2PKH
            | MAINNET_P2SH
            => BitcoinNetworkType::Mainnet,
            | TESTNET_P2PKH
            | TESTNET_P2SH
            => BitcoinNetworkType::Testnet,
            _=> panic!("unexpected prefix byte"),
        }
    }

    /// Returns the transaction type.
    pub fn transaction_type(&self) -> BitcoinTransactionType {
        match self.bytes[0] {
            | MAINNET_P2PKH
            | TESTNET_P2PKH
            => BitcoinTransactionType::P2pkh,
            | MAINNET_P2SH
            | TESTNET_P2SH
            => BitcoinTransactionType::P2sh,
            _=> panic!("unexpected prefix byte"),
        }
    }

    /// Returns the 20 byte hash that this address represents.
    pub fn hash_bytes(&self) -> &[u8] {
        &self.bytes[1..]
    }

    /// Creates a Bitcoin address from its Base58Check string representation.
    pub fn base58(address: &str) -> Self {
        let mut bytes: [u8; 21] = [0_u8; 21];
        let data = Base58CheckBitcoinEncoding::decode(21, address).unwrap();

        bytes.clone_from_slice(&data);

        Self {
            bytes: bytes
        }
    }

    /// Creates a Bitcoin address for some 20 byte digest.
    pub fn for_hash_bytes(prefix: BitcoinEncodingPrefix, hash_bytes: &[u8]) -> Self {
        let mut bytes: [u8; 21] = [0_u8; 21];
        let prefix_bytes: &[u8] = BitcoinEncodingPrefix::bytes(prefix);

        bytes[0..=0].clone_from_slice(prefix_bytes);
        bytes[1..=20].clone_from_slice(&hash_bytes);

        Self {
            bytes: bytes
        }
    }

    /// Creates a P2PKH Bitcoin address for a given (public key) elliptic curve point.
    ///
    /// This is encoded using a digest of the compressed point SEC format
    /// (`CompressedPointSecFormatBytes`).
    pub fn for_compressed_point(network: BitcoinNetworkType, point_bytes: CompressedPointSecFormatBytes) -> Self {
        let mut bytes: [u8; 21] = [0_u8; 21];
        let prefix_bytes: &[u8] = BitcoinEncodingPrefix::bytes(Self::base58_encoding_type(network, BitcoinTransactionType::P2pkh));

        bytes[0..=0].clone_from_slice(prefix_bytes);
        bytes[1..=20].clone_from_slice(&hash_160(point_bytes.bytes()));

        Self {
            bytes: bytes
        }
    }

    /// Creates a P2PKH Bitcoin address for a given (public key) elliptic curve point.
    ///
    /// This is encoded using a hash of the uncompressed point SEC format
    /// (`UncompressedPointSecFormatBytes`).
    pub fn for_uncompressed_point(network: BitcoinNetworkType, point_bytes: UncompressedPointSecFormatBytes) -> Self {
        let mut bytes: [u8; 21] = [0_u8; 21];
        let prefix_bytes: &[u8] = BitcoinEncodingPrefix::bytes(Self::base58_encoding_type(network, BitcoinTransactionType::P2pkh));

        bytes[0..=0].clone_from_slice(prefix_bytes);
        bytes[1..=20].clone_from_slice(&hash_160(point_bytes.bytes()));

        Self {
            bytes: bytes
        }
    }

    /// Creates a P2SH Bitcoin address for a given redeem script.
    ///
    /// This is encoded using a digest of the script bytes.
    pub fn for_redeem_script(network: BitcoinNetworkType, redeem_script_bytes: ScriptBytes) -> Self {
        let mut bytes: [u8; 21] = [0_u8; 21];
        let prefix_bytes: &[u8] = BitcoinEncodingPrefix::bytes(Self::base58_encoding_type(network, BitcoinTransactionType::P2sh));

        bytes[0..=0].clone_from_slice(prefix_bytes);
        bytes[1..=20].clone_from_slice(&hash_160(redeem_script_bytes.bytes()));

        Self {
            bytes: bytes
        }
    }
}
