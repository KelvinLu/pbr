//! SEC formats for ECDSA elliptic curve points.

use crate::math::elliptic_curve::point::EllipticCurvePoint;
use crate::math::elliptic_curve::point::PointNotOnCurveError;
use crate::math::elliptic_curve::rhs::solve_rhs_finite_field;
use crate::math::elliptic_curve::rhs::DerivationError;
use crate::math::finite_field_element::FiniteFieldElement;
use crate::math::number::Number;
use crate::util::byte_string::ByteString;
use crate::util::byte_string::ByteSlice;
use crate::util::hexadecimal::hexadecimal_encode;
use crate::util::number::U256;

/// SEC encoding for uncompressed ECDSA points.
///
/// # Format
///
/// 1. `0x04` - marker byte
/// 2. (32 bytes) - x coordinate, big-endian
/// 2. (32 bytes) - y coordinate, big-endian
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct UncompressedPointSecFormatBytes {
    bytes: [u8; 65]
}

impl ByteString for UncompressedPointSecFormatBytes {
    fn of(bytes: &[u8]) -> Self {
        let mut buffer = [0_u8; 65];

        buffer.clone_from_slice(bytes);

        assert_eq!(buffer[0], 4_u8);

        Self { bytes: buffer }
    }

}

impl ByteSlice for UncompressedPointSecFormatBytes {
    fn bytes(&self) -> &[u8] { &self.bytes }
}

impl std::fmt::Display for UncompressedPointSecFormatBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut buffer = [0_u8; 65 * 2];

        hexadecimal_encode(self.bytes(), &mut buffer).unwrap();
        write!(f, "<SEC format uncompressed elliptic curve point {}>", std::str::from_utf8(&buffer).unwrap())
    }
}

impl From<&EllipticCurvePoint> for UncompressedPointSecFormatBytes {
    /// Produces a SEC format byte representation of an elliptic curve point.
    fn from(point: &EllipticCurvePoint) -> Self {
        let mut bytes: [u8; 65] = [0; 65];

        let (x, y) = coordinates(point);

        bytes[0] = 4_u8;
        bytes[1..=32].clone_from_slice(&coordinate_bytes(x));
        bytes[33..=64].clone_from_slice(&coordinate_bytes(y));

        Self { bytes: bytes }
    }
}

impl UncompressedPointSecFormatBytes {
    /// Produces an elliptic curve point from a SEC format byte representation.
    pub fn elliptic_curve_point(
        &self,
        elliptic_curve_a: U256,
        elliptic_curve_b: U256,
        finite_field_order: U256
    ) -> Result<EllipticCurvePoint, PointNotOnCurveError> {
        assert_eq!(self.bytes[0], 4_u8);

        let mut x: [u8; 32] = [0; 32];
        x.clone_from_slice(&self.bytes[1..=32]);

        let x = FiniteFieldElement::new(U256::from_be_bytes(x), finite_field_order);

        let mut y: [u8; 32] = [0; 32];
        y.clone_from_slice(&self.bytes[33..=64]);

        let y = FiniteFieldElement::new(U256::from_be_bytes(y), finite_field_order);

        let a = FiniteFieldElement::new(elliptic_curve_a, finite_field_order);
        let b = FiniteFieldElement::new(elliptic_curve_b, finite_field_order);

        EllipticCurvePoint::try_from(((x, y), (a, b)))
    }
}

/// SEC encoding for compressed ECDSA points.
///
/// # Format
///
/// 1. `0x02` or `0x03` - marker byte (`0x02` denotes an even y coordinate, and `0x03` denotes an
///    odd x coordinate)
/// 2. (32 bytes) - x coordinate, big-endian
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct CompressedPointSecFormatBytes {
    bytes: [u8; 33]
}

impl ByteString for CompressedPointSecFormatBytes {
    fn of(bytes: &[u8]) -> Self {
        let mut buffer = [0_u8; 33];

        buffer.clone_from_slice(bytes);
        Self { bytes: buffer }
    }
}

impl ByteSlice for CompressedPointSecFormatBytes {
    fn bytes(&self) -> &[u8] { &self.bytes }
}

impl std::fmt::Display for CompressedPointSecFormatBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut buffer = [0_u8; 33 * 2];

        hexadecimal_encode(self.bytes(), &mut buffer).unwrap();
        write!(f, "<SEC format compressed elliptic curve point {}>", std::str::from_utf8(&buffer).unwrap())
    }
}

impl From<&EllipticCurvePoint> for CompressedPointSecFormatBytes {
    /// Produces a SEC format byte representation of an elliptic curve point.
    fn from(point: &EllipticCurvePoint) -> Self {
        let mut bytes: [u8; 33] = [0; 33];

        let (x, y) = coordinates(point);

        bytes[0] = if coordinate_is_even(y) { 2_u8 } else { 3_u8 };
        bytes[1..=32].clone_from_slice(&coordinate_bytes(x));

        Self { bytes: bytes }
    }
}

impl CompressedPointSecFormatBytes {
    /// Produces an elliptic curve point from a SEC format byte representation.
    ///
    /// `quadratic_residue_roots` is a closure that is used to solve for the right-hand side of
    /// `y^2 = x^3 + ax + b`, should Euler's criteron indicate that there exists square roots.
    pub fn elliptic_curve_point<F>(
        &self,
        elliptic_curve_a: U256,
        elliptic_curve_b: U256,
        finite_field_order: U256,
        quadratic_residue_roots: F
    ) -> Result<EllipticCurvePoint, DerivationError>
    where F: Fn(U256) -> (U256, U256) {
        let y_is_even: bool = match self.bytes[0] {
            2_u8 => true,
            3_u8 => false,
            _ => panic!("unexpected header byte")
        };

        let mut x: [u8; 32] = [0; 32];
        x.clone_from_slice(&self.bytes[1..=32]);

        let x = U256::from_be_bytes(x);

        let (point_1, point_2) = solve_rhs_finite_field(
            x,
            elliptic_curve_a,
            elliptic_curve_b,
            finite_field_order,
            quadratic_residue_roots
        )?;

        match point_1 {
            EllipticCurvePoint::PointOnCurve(p) => {
                match (y_is_even, coordinate_is_even(p.y)) {
                    (true, true) => Ok(point_1),
                    (true, false) => Ok(point_2),
                    (false, true) => Ok(point_2),
                    (false, false) => Ok(point_1),
                }
            },
            EllipticCurvePoint::PointAtInfinity => panic!("unexpected point at infinity")
        }
    }
}

fn coordinates(point: &EllipticCurvePoint) -> (Number, Number) {
    let EllipticCurvePoint::PointOnCurve(point) = *point else { panic!("cannot serialize point at infinity") };

    (point.x, point.y)
}

fn coordinate_value(number: Number) -> U256 {
    match number {
        Number::FiniteFieldElement(element) => element.value,
        _ => panic!("unsupported type")
    }
}

fn coordinate_bytes(number: Number) -> [u8; 32] {
    coordinate_value(number).to_be_bytes()
}

fn coordinate_is_even(number: Number) -> bool {
    !coordinate_value(number).bit(0)
}
