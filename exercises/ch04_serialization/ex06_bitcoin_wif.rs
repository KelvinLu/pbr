use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use serialization::bitcoin_base58check::Base58CheckBitcoinEncoding;
use serialization::bitcoin_base58check::BitcoinEncodingPrefix;
use serialization::bitcoin_base58check::BitcoinEncodingSuffix;
use util::number::U256;
use util::number::uint;

pub fn exercise() {
    prompt("Base58 encoding (Bitcoin WIF)");

    section("(5003) * <secp256k1 G>");

    let secret_e = U256::from(5003);
    let secret_bytes: [u8; 32] = secret_e.to_be_bytes();

    let mut bytes: [u8; 34] = [0_u8; 34];
    bytes[0..=0].clone_from_slice(BitcoinEncodingPrefix::bytes(BitcoinEncodingPrefix::TestnetWifPrivateKey));
    bytes[1..=32].clone_from_slice(&secret_bytes);
    bytes[32..=32].clone_from_slice(BitcoinEncodingSuffix::bytes(BitcoinEncodingSuffix::WifWithCompressedPoint));

    show_display(&Base58CheckBitcoinEncoding::from_bytes(&bytes));

    section("(2021 ^ 5) * <secp256k1 G>");

    let secret_e = U256::from(2021).pow(U256::from(5));
    let secret_bytes: [u8; 32] = secret_e.to_be_bytes();

    let mut bytes: [u8; 33] = [0_u8; 33];
    bytes[0..=0].clone_from_slice(BitcoinEncodingPrefix::bytes(BitcoinEncodingPrefix::TestnetWifPrivateKey));
    bytes[1..=32].clone_from_slice(&secret_bytes);

    show_display(&Base58CheckBitcoinEncoding::from_bytes(&bytes));

    section("(0x54321deadbeef) * <secp256k1 G>");

    let secret_e = uint!(0x54321deadbeef_U256);
    let secret_bytes: [u8; 32] = secret_e.to_be_bytes();

    let mut bytes: [u8; 34] = [0_u8; 34];
    bytes[0..=0].clone_from_slice(BitcoinEncodingPrefix::bytes(BitcoinEncodingPrefix::TestnetWifPrivateKey));
    bytes[1..=32].clone_from_slice(&secret_bytes);
    bytes[32..=32].clone_from_slice(BitcoinEncodingSuffix::bytes(BitcoinEncodingSuffix::WifWithCompressedPoint));

    show_display(&Base58CheckBitcoinEncoding::from_bytes(&bytes));
}
