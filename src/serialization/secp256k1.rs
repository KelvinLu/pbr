//! secp256k1 elliptic curve points serialization helpers.

use crate::serialization::point::UncompressedPointSecFormatBytes;
use crate::serialization::point::CompressedPointSecFormatBytes;
use crate::crypto::secp256k1::{Secp256k1Point, ELLIPTIC_CURVE_A, ELLIPTIC_CURVE_B, FINITE_FIELD_ORDER};
use crate::math::algorithm::tonelli_shanks;
use crate::math::elliptic_curve::point::PointNotOnCurveError;
use crate::math::elliptic_curve::rhs::DerivationError;

impl UncompressedPointSecFormatBytes {
    /// Produces a secp256k1 point from a SEC format byte representation.
    pub fn elliptic_curve_point_secp256k1(&self) -> Result<Secp256k1Point, PointNotOnCurveError> {
        let point = self.elliptic_curve_point(
            ELLIPTIC_CURVE_A,
            ELLIPTIC_CURVE_B,
            FINITE_FIELD_ORDER,
        )?;

        Ok(Secp256k1Point::Point(point))
    }
}

impl CompressedPointSecFormatBytes {
    /// Produces a secp256k1 point from a SEC format byte representation.
    pub fn elliptic_curve_point_secp256k1(&self) -> Result<Secp256k1Point, DerivationError> {
        let point = self.elliptic_curve_point(
            ELLIPTIC_CURVE_A,
            ELLIPTIC_CURVE_B,
            FINITE_FIELD_ORDER,
            |y_squared| tonelli_shanks::square_roots(y_squared, FINITE_FIELD_ORDER).unwrap()
        )?;

        Ok(Secp256k1Point::Point(point))
    }
}
