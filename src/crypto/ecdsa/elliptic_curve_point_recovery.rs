//! Elliptic curve point derivation for ECDSA signature values.

use crate::crypto::ecdsa::signature::Signature;
use crate::math::elliptic_curve::point::EllipticCurvePoint;
use crate::math::elliptic_curve::rhs::solve_rhs_finite_field;
use crate::math::elliptic_curve::rhs::DerivationError;
use crate::util::number::U256;

/// Elliptic curve point derivation.
impl Signature {
    /// Produces a pair of elliptic curve points that may verify the signature against some given
    /// `data` value.
    ///
    /// This relies on an underlying elliptic curve and a generator point.
    ///
    /// `quadratic_residue_roots` is a closure that is used to solve for the right-hand side of
    /// `y^2 = x^3 + ax + b`, should Euler's criteron indicate that there exists square roots.
    ///
    /// Note that although this returns two points (one R point is solved for, giving two reflected
    /// points), there may be more points (`2 * (h + 1)`) depending on the cofactor `h` of the
    /// elliptic curve. These additional points may be sought by adjusting the r-value of the
    /// signature and recalculating.
    pub fn derive_points<F>(
        &self,
        data: U256,
        generator_point: EllipticCurvePoint,
        elliptic_curve_order: U256,
        elliptic_curve_a: U256,
        elliptic_curve_b: U256,
        finite_field_order: U256,
        quadratic_residue_roots: F
    ) -> Result<(EllipticCurvePoint, EllipticCurvePoint), DerivationError>
    where F: Fn(U256) -> (U256, U256) {
        let x = self.r;

        let (r_point_1, r_point_2) = solve_rhs_finite_field(
            x,
            elliptic_curve_a,
            elliptic_curve_b,
            finite_field_order,
            quadratic_residue_roots
        ).unwrap();

        let r_inv = self.r.pow_mod(elliptic_curve_order - U256::from(2), elliptic_curve_order);

        let point_1 = r_inv * ((self.s * r_point_1) - (data * generator_point));
        let point_2 = r_inv * ((self.s * r_point_2) - (data * generator_point));

        Ok((point_1, point_2))
    }

    /// Verify the signature against some `data` value.
    ///
    /// This relies on an underlying elliptic curve and a generator point.
    ///
    /// The point used for validation will be derived from the signature's r-value.
    ///
    /// `quadratic_residue_roots` is a closure that is used to solve for the right-hand side of
    /// `y^2 = x^3 + ax + b`, should Euler's criteron indicate that there exists square roots.
    pub fn verify_by_derivation<F>(
        &self,
        data: U256,
        generator_point: EllipticCurvePoint,
        elliptic_curve_order: U256,
        elliptic_curve_a: U256,
        elliptic_curve_b: U256,
        finite_field_order: U256,
        quadratic_residue_roots: F
    ) -> bool
    where F: Fn(U256) -> (U256, U256) {
        let points = self.derive_points(
            data,
            generator_point,
            elliptic_curve_order,
            elliptic_curve_a,
            elliptic_curve_b,
            finite_field_order,
            quadratic_residue_roots
        );

        let point = match points {
            Ok(points) => points.0,
            Err(_) => return false,
        };

        let EllipticCurvePoint::PointOnCurve(_) = point else {
            return false;
        };

        self.verify_point(data, point, generator_point, elliptic_curve_order)
    }
}
