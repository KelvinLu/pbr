use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use serialization::bitcoin_address::BitcoinAddress;
use serialization::point::CompressedPointSecFormatBytes;
use bitcoin::network::BitcoinNetworkType;
use crypto::secp256k1::Secp256k1Point;
use crypto::digest::hash_256;
use math::elliptic_curve::point::EllipticCurvePoint;
use util::hexadecimal::hexadecimal_encode;
use util::number::U256;

pub fn exercise() {
    prompt("Testnet address");

    let secret_e = U256::from_be_bytes(hash_256(b"my little secret"));

    section("(SHA-256(SHA-256(<secret phrase>))) * <secp256k1 G>");

    let point: EllipticCurvePoint = (secret_e * Secp256k1Point::generator_point()).into();
    let point_bytes = CompressedPointSecFormatBytes::from(&point);
    let address = BitcoinAddress::for_compressed_point(BitcoinNetworkType::Testnet, point_bytes);

    show_display(&point);
    show_display(&point_bytes);

    section("Bitcoin address");

    show_display(&address);

    section("Details");

    let mut hash_hexadecimal: [u8; 40] = [0_u8; 40];
    hexadecimal_encode(&address.hash_bytes(), &mut hash_hexadecimal).unwrap();

    message(&format!("20 byte hash: {}", std::str::from_utf8(&hash_hexadecimal).unwrap()));
    show_debug(&address.network());
    show_debug(&address.transaction_type());
}
