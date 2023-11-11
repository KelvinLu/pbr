use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use serialization::point::UncompressedPointSecFormatBytes;
use crypto::secp256k1::Secp256k1Point;
use math::elliptic_curve::point::EllipticCurvePoint;
use util::number::U256;
use util::number::uint;

pub fn exercise() {
    prompt("Uncompressed SEC format");

    section("(5000) * <secp256k1 G>");

    let point: EllipticCurvePoint = (U256::from(5000) * Secp256k1Point::generator_point()).into();
    let bytes = UncompressedPointSecFormatBytes::from(&point);

    show_display(&point);
    show_display(&bytes);

    section("(2018 ^ 5) * <secp256k1 G>");

    let point: EllipticCurvePoint = (U256::from(2018).pow(U256::from(5)) * Secp256k1Point::generator_point()).into();
    let bytes = UncompressedPointSecFormatBytes::from(&point);

    show_display(&point);
    show_display(&bytes);

    section("(0xdeadbeef12345) * <secp256k1 G>");

    let point: EllipticCurvePoint = (uint!(0xdeadbeef12345_U256) * Secp256k1Point::generator_point()).into();
    let bytes = UncompressedPointSecFormatBytes::from(&point);

    show_display(&point);
    show_display(&bytes);
}
