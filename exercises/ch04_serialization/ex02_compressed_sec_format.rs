use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use serialization::point::CompressedPointSecFormatBytes;
use crypto::secp256k1::Secp256k1Point;
use math::elliptic_curve::point::EllipticCurvePoint;
use util::number::U256;
use util::number::uint;

pub fn exercise() {
    prompt("Compressed SEC format");

    section("(5001) * <secp256k1 G>");

    let point: EllipticCurvePoint = (U256::from(5001) * Secp256k1Point::generator_point()).into();
    let bytes = CompressedPointSecFormatBytes::from(&point);

    show_display(&point);
    show_display(&bytes);

    section("(2019 ^ 5) * <secp256k1 G>");

    let point: EllipticCurvePoint = (U256::from(2019).pow(U256::from(5)) * Secp256k1Point::generator_point()).into();
    let bytes = CompressedPointSecFormatBytes::from(&point);

    show_display(&point);
    show_display(&bytes);

    section("(0xdeadbeef54321) * <secp256k1 G>");

    let point: EllipticCurvePoint = (uint!(0xdeadbeef54321_U256) * Secp256k1Point::generator_point()).into();
    let bytes = CompressedPointSecFormatBytes::from(&point);

    show_display(&point);
    show_display(&bytes);
}
