//! Right-hand side of the elliptic curve equation.

use crate::math::elliptic_curve::point::EllipticCurvePoint;
use crate::math::elliptic_curve::point::PointNotOnCurveError;
use crate::math::finite_field_element::FiniteFieldElement;
use crate::math::algorithm::eulers_criterion;
use crate::util::number::U256;

/// Error given when elliptic curve point derivation fails.
#[derive(Debug)]
pub enum DerivationError {
    NoQuadratricResidue,
    PointOnCurve(PointNotOnCurveError),
}

impl From<PointNotOnCurveError> for DerivationError {
    fn from(error: PointNotOnCurveError) -> Self {
        Self::PointOnCurve(error)
    }
}

/// Solves the right-hand side of the elliptic curve equation within a finite field, given an x
/// coordinate, and returns the two resulting points.
///
/// `quadratic_residue_roots` is a closure that is used to solve for the right-hand side of
/// `y^2 = x^3 + ax + b`, should Euler's criteron indicate that there exists square roots.
pub fn solve_rhs_finite_field<F>(
    x_coordinate: U256,
    elliptic_curve_a: U256,
    elliptic_curve_b: U256,
    finite_field_order: U256,
    quadratic_residue_roots: F
) -> Result<(EllipticCurvePoint, EllipticCurvePoint), DerivationError>
where F: Fn(U256) -> (U256, U256) {
    let a = FiniteFieldElement::new(elliptic_curve_a, finite_field_order);
    let b = FiniteFieldElement::new(elliptic_curve_b, finite_field_order);

    let x = FiniteFieldElement::new(x_coordinate, finite_field_order);

    let y_squared = x.pow(U256::from(3)) + (a * x) + b;

    let (y_1, y_2) =
        if eulers_criterion::quadratic_residue(y_squared.value, y_squared.cardinality) {
            let (y_1, y_2) = quadratic_residue_roots(y_squared.value);

            (FiniteFieldElement::new(y_1, finite_field_order), FiniteFieldElement::new(y_2, finite_field_order))
        } else {
            return Err(DerivationError::NoQuadratricResidue)
        };

    let point_1 = EllipticCurvePoint::try_from(((x, y_1), (a, b)))?;
    let point_2 = EllipticCurvePoint::try_from(((x, y_2), (a, b)))?;

    Ok((point_1, point_2))
}
