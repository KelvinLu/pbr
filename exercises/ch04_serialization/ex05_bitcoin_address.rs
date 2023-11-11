use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use serialization::bitcoin_base58check::Base58CheckBitcoinEncoding;
use serialization::bitcoin_base58check::BitcoinEncodingPrefix;
use serialization::point::UncompressedPointSecFormatBytes;
use serialization::point::CompressedPointSecFormatBytes;
use crypto::secp256k1::Secp256k1Point;
use crypto::digest::hash_160;
use math::elliptic_curve::point::EllipticCurvePoint;
use util::byte_string::ByteSlice;
use util::number::U256;
use util::number::uint;

pub fn exercise() {
    prompt("Base58 encoding (Bitcoin address)");

    section("(5002) * <secp256k1 G>");

    let point: EllipticCurvePoint = (U256::from(5002) * Secp256k1Point::generator_point()).into();
    let point_bytes = UncompressedPointSecFormatBytes::from(&point);

    show_display(&point);
    show_display(&point_bytes);

    let mut bytes: [u8; 21] = [0_u8; 21];
    bytes[0..=0].clone_from_slice(BitcoinEncodingPrefix::bytes(BitcoinEncodingPrefix::TestnetP2pkhAddress));
    bytes[1..=20].clone_from_slice(&hash_160(point_bytes.bytes()));

    show_display(&Base58CheckBitcoinEncoding::from_bytes(&bytes));

    section("(2020 ^ 5) * <secp256k1 G>");

    let point: EllipticCurvePoint = (U256::from(2020).pow(U256::from(5)) * Secp256k1Point::generator_point()).into();
    let point_bytes = CompressedPointSecFormatBytes::from(&point);

    show_display(&point);
    show_display(&point_bytes);

    let mut bytes: [u8; 21] = [0_u8; 21];
    bytes[0..=0].clone_from_slice(BitcoinEncodingPrefix::bytes(BitcoinEncodingPrefix::TestnetP2pkhAddress));
    bytes[1..=20].clone_from_slice(&hash_160(point_bytes.bytes()));

    show_display(&Base58CheckBitcoinEncoding::from_bytes(&bytes));

    section("(0x12345deadbeef) * <secp256k1 G>");

    let point: EllipticCurvePoint = (uint!(0x12345deadbeef_U256) * Secp256k1Point::generator_point()).into();
    let point_bytes = CompressedPointSecFormatBytes::from(&point);

    show_display(&point);
    show_display(&point_bytes);

    let mut bytes: [u8; 21] = [0_u8; 21];
    bytes[0..=0].clone_from_slice(BitcoinEncodingPrefix::bytes(BitcoinEncodingPrefix::MainnetP2pkhAddress));
    bytes[1..=20].clone_from_slice(&hash_160(point_bytes.bytes()));

    show_display(&Base58CheckBitcoinEncoding::from_bytes(&bytes));
}
