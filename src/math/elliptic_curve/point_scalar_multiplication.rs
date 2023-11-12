//! Elliptic curve point scalar multiplication.

use std::ops;

use crate::math::elliptic_curve::point::EllipticCurvePoint;
use crate::util::number::U256;

/// Point scalar multiplication.
impl ops::Mul<EllipticCurvePoint> for U256 {
    type Output = EllipticCurvePoint;

    /// Perform scalar multiplication on a point, given a whole number.
    fn mul(self, rhs: EllipticCurvePoint) -> Self::Output {
        let mut coefficient = self;
        let mut accumulator = EllipticCurvePoint::PointAtInfinity;
        let mut base = rhs;

        while coefficient > U256::ZERO {
            if coefficient.bit(0) { accumulator = accumulator + base }
            base = base + base;
            coefficient = coefficient >> 1;
        }

        accumulator
    }
}

/// Point scalar multiplication.
impl ops::Mul<EllipticCurvePoint> for u32 {
    type Output = EllipticCurvePoint;

    /// Perform scalar multiplication on a point, given a whole number.
    fn mul(self, rhs: EllipticCurvePoint) -> Self::Output {
        U256::from(self) * rhs
    }
}

/// Point scalar multiplication.
impl ops::Mul<EllipticCurvePoint> for i32 {
    type Output = EllipticCurvePoint;

    /// Perform scalar multiplication on a point, given an integer.
    fn mul(self, rhs: EllipticCurvePoint) -> Self::Output {
        U256::from(self) * rhs
    }
}
