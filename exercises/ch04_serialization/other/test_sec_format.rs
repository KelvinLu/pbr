use programming_bitcoin_in_rust::*;

use serialization::point::UncompressedPointSecFormatBytes;
use serialization::point::CompressedPointSecFormatBytes;
use crypto::secp256k1;
use math::elliptic_curve::point::EllipticCurvePoint;
use util::byte_string::ByteString;
use util::byte_string::ByteSlice;
use util::hexadecimal::hexadecimal_encode;
use util::hexadecimal::hexadecimal_string;
use util::number::U256;

pub fn run() {
    let mut buffer_65_byte_hexadecimal_string = [0_u8; 65 * 2];
    let mut buffer_33_byte_hexadecimal_string = [0_u8; 33 * 2];
    let mut buffer_65_bytes = [0_u8; 65];
    let mut buffer_33_bytes = [0_u8; 33];

    for (secret_e, uncompressed, compressed) in [
        (
            U256::from(123),
            "04a598a8030da6d86c6bc7f2f5144ea549d28211ea58faa70ebf4c1e665c1fe9b5204b5d6f84822c307e4b4a7140737aec23fc63b65b35f86a10026dbd2d864e6b",
            "03a598a8030da6d86c6bc7f2f5144ea549d28211ea58faa70ebf4c1e665c1fe9b5"
        ),
        (
            U256::from(999).pow(U256::from(3)),
            "049d5ca49670cbe4c3bfa84c96a8c87df086c6ea6a24ba6b809c9de234496808d56fa15cc7f3d38cda98dee2419f415b7513dde1301f8643cd9245aea7f3f911f9",
            "039d5ca49670cbe4c3bfa84c96a8c87df086c6ea6a24ba6b809c9de234496808d5"
        ),
        (
            U256::from(2019).pow(U256::from(5)),
            "04933ec2d2b111b92737ec12f1c5d20f3233a0ad21cd8b36d0bca7a0cfa5cb870196cbbfdd572f75ace44d0aa59fbab6326cb9f909385dcd066ea27affef5a488c",
            "02933ec2d2b111b92737ec12f1c5d20f3233a0ad21cd8b36d0bca7a0cfa5cb8701"
        )
    ] {
        let point: EllipticCurvePoint = (secret_e * secp256k1::Secp256k1Point::generator_point()).into();

        let point_bytes = UncompressedPointSecFormatBytes::from(&point);

        assert_eq!(hexadecimal_encode(point_bytes.bytes(), &mut buffer_65_byte_hexadecimal_string).unwrap(), uncompressed.as_bytes());

        hexadecimal_string(uncompressed, &mut buffer_65_bytes).unwrap();

        let point_bytes = UncompressedPointSecFormatBytes::of(&buffer_65_bytes);
        let point_from_bytes = point_bytes.elliptic_curve_point_secp256k1().unwrap();

        assert_eq!(point, point_from_bytes.into());

        let point_bytes = CompressedPointSecFormatBytes::from(&point);

        assert_eq!(hexadecimal_encode(point_bytes.bytes(), &mut buffer_33_byte_hexadecimal_string).unwrap(), compressed.as_bytes());

        hexadecimal_string(compressed, &mut buffer_33_bytes).unwrap();

        let point_bytes = CompressedPointSecFormatBytes::of(&buffer_33_bytes);
        let point_from_bytes = point_bytes.elliptic_curve_point_secp256k1().unwrap();

        assert_eq!(point, point_from_bytes.into());
    }
}
