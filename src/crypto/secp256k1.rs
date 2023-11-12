//! Elliptic curve secp256k1.

use std::ops;

use crate::math::elliptic_curve::point::EllipticCurvePoint;
use crate::math::finite_field_element::FiniteFieldElement;
use crate::crypto::ecdsa::signature::Signature;
use crate::crypto::ecdsa::elliptic_curve_point_recovery::DerivationError;
use crate::math::algorithm::tonelli_shanks;
use crate::util::number::U256;
use crate::util::number::uint;

uint! {
    /// secp256k1 finite field order.
    ///
    /// `p = (2 ^ 256) - (2 ^ 32) - 977`
    pub const FINITE_FIELD_ORDER: U256 = 0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f_U256;

    /// secp256k1 elliptic curve parameter `a`.
    ///
    /// `a = 0`
    pub const ELLIPTIC_CURVE_A: U256 = 0_U256;

    /// secp256k1 elliptic curve parameter `b`.
    ///
    /// `a = 7`
    pub const ELLIPTIC_CURVE_B: U256 = 7_U256;

    /// secp256k1 generator point coordinates (`x` coordinate).
    pub const GENERATOR_POINT_X: U256 = 0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798_U256;

    /// secp256k1 generator point coordinates (`y` coordinate).
    pub const GENERATOR_POINT_Y: U256 = 0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8_U256;

    /// secp256k1 elliptic curve cyclic group order.
    pub const ELLIPTIC_CURVE_ORDER: U256 = 0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141_U256;
}


/// A point on the secp256k1 elliptic curve.
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum Secp256k1Point {
    Point(EllipticCurvePoint)
}

impl std::fmt::Display for Secp256k1Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Self::Point(point) = self;

        match point {
            EllipticCurvePoint::PointOnCurve(point) => write!(f, "secp256k1 point ({}, {})", point.x, point.y),
            EllipticCurvePoint::PointAtInfinity => write!(f, "secp256k1 point at infinity")
        }
    }
}

impl Secp256k1Point {
    /// Create a secp256k1 point.
    pub fn new(x: U256, y: U256) -> Self {
        let x = FiniteFieldElement::new(x, FINITE_FIELD_ORDER);
        let y = FiniteFieldElement::new(y, FINITE_FIELD_ORDER);
        let a = FiniteFieldElement::new(ELLIPTIC_CURVE_A, FINITE_FIELD_ORDER);
        let b = FiniteFieldElement::new(ELLIPTIC_CURVE_B, FINITE_FIELD_ORDER);

        Self::Point(EllipticCurvePoint::try_from(((x, y), (a, b))).unwrap())
    }

    /// Returns the secp256k1 generator point.
    pub fn generator_point() -> Self {
        Self::new(GENERATOR_POINT_X, GENERATOR_POINT_Y)
    }
}

/// Point addition.
impl ops::Add<Self> for Secp256k1Point {
    type Output = Self;

    /// Adds another secp256k1 point.
    fn add(self, rhs: Self) -> Self::Output {
        let Self::Point(point_1) = self;
        let Self::Point(point_2) = rhs;

        Self::Point(point_1 + point_2)
    }
}

/// Point scalar multiplication.
impl ops::Mul<Secp256k1Point> for U256 {
    type Output = Secp256k1Point;

    /// Perform scalar multiplication on a secp256k1 point, given a whole number.
    fn mul(self, rhs: Secp256k1Point) -> Self::Output {
        let Secp256k1Point::Point(point) = rhs;
        let n = self % ELLIPTIC_CURVE_ORDER;

        Secp256k1Point::Point(n * point)
    }
}

impl From<Secp256k1Point> for EllipticCurvePoint {
    /// Returns the underlying `EllipticCurvePoint`.
    fn from(point: Secp256k1Point) -> Self {
        let Secp256k1Point::Point(point) = point;

        point
    }
}

/// ECDSA signature operations over the secp256k1 elliptic curve.
impl Signature {
    /// Produce a signature for some `data` value, signed using a (private key) `secret_e`.
    ///
    /// `data` and `secret_e` may be expressed as 256 bit arguments (32 bytes).
    ///
    /// The corresponding (public key) point is `secret_e * Secp256k1Point::generator_point()`,
    /// which may be used to verify this signature against the original `data`.
    ///
    /// The random commitment is chosen in a deterministic manner, based on RFC 6979. It utilizes
    /// HMAC digests (SHA-256) against the given `data` and `secret_e`, and assumes the `data` has
    /// already been hashed through a similar 256 bit digest.
    pub fn sign_secp256k1(data: U256, secret_e: U256) -> Self {
        Self::sign(data, secret_e, Secp256k1Point::generator_point().into(), ELLIPTIC_CURVE_ORDER)
    }

    /// Produce a signature for some `data` value, signed using a (private key) `secret_e` against
    /// a random commitment (`random_commitment`).
    ///
    /// `data`, `secret_e`, and `random_commitment` may be expressed as 256 bit arguments (32 bytes).
    ///
    /// The corresponding (public key) point is `secret_e * Secp256k1Point::generator_point()`,
    /// which may be used to verify this signature against the original `data`.
    pub fn new_secp256k1(data: U256, secret_e: U256, random_commitment: U256) -> Self {
        Self::new(data, secret_e, random_commitment, Secp256k1Point::generator_point().into(), ELLIPTIC_CURVE_ORDER)
    }

    /// Verify the signature against some `data` value, given a secp256k1 `point`.
    pub fn verify_point_secp256k1(&self, data: U256, point: Secp256k1Point) -> bool {
        self.verify_point(data, point.into(), Secp256k1Point::generator_point().into(), ELLIPTIC_CURVE_ORDER)
    }

    /// Derive the elliptic curve points that may be used to validate the signature against some
    /// given `data`.
    ///
    /// secp256k1 points utilize the Tonelli-Shanks algorithm to find the square roots of some
    /// value within the finite field.
    pub fn derive_points_secp256k1(&self, data: U256) -> Result<DerivedSecp256k1Points, DerivationError> {
        let (point_1, point_2) = self.derive_points(
            data,
            Secp256k1Point::generator_point().into(),
            ELLIPTIC_CURVE_ORDER,
            ELLIPTIC_CURVE_A,
            ELLIPTIC_CURVE_B,
            FINITE_FIELD_ORDER,
            |y_squared| tonelli_shanks::square_roots(y_squared, FINITE_FIELD_ORDER).unwrap()
        )?;

        if self.r >= FINITE_FIELD_ORDER - ELLIPTIC_CURVE_ORDER {
            Ok(
                DerivedSecp256k1Points::Pair(
                    [Secp256k1Point::Point(point_1), Secp256k1Point::Point(point_2)]
                )
            )
        } else {
            // For a given secp256k1 signature, with finite field order p and elliptic curve order n, ...
            // ... if R.x < (p - n) (recall R is the point with x coordinate referred to as r),
            // ... then another pair of points with x coordinate (r + n) may also exist.
            //
            // The likelihood of this is (2 * (p - n) / p) =~ 7 * (10 ^ -39) -- extremely rare.
            let signature = Signature {
                r: self.r + ELLIPTIC_CURVE_ORDER,
                s: self.s,
            };

            let (point_3, point_4) = signature.derive_points(
                data,
                Secp256k1Point::generator_point().into(),
                ELLIPTIC_CURVE_ORDER,
                ELLIPTIC_CURVE_A,
                ELLIPTIC_CURVE_B,
                FINITE_FIELD_ORDER,
                |y_squared| tonelli_shanks::square_roots(y_squared, FINITE_FIELD_ORDER).unwrap()
            )?;

            Ok(
                DerivedSecp256k1Points::Quartet(
                    [
                        Secp256k1Point::Point(point_1),
                        Secp256k1Point::Point(point_2),
                        Secp256k1Point::Point(point_3),
                        Secp256k1Point::Point(point_4),
                    ]
                )
            )
        }
    }

    /// Verify the signature against some `data` value.
    ///
    /// The point used for validation will be derived from the signature's r-value.
    pub fn verify_by_derivation_secp256k1(&self, data: U256) -> bool {
        self.verify_by_derivation(
            data,
            Secp256k1Point::generator_point().into(),
            ELLIPTIC_CURVE_ORDER,
            ELLIPTIC_CURVE_A,
            ELLIPTIC_CURVE_B,
            FINITE_FIELD_ORDER,
            |y_squared| tonelli_shanks::square_roots(y_squared, FINITE_FIELD_ORDER).unwrap()
        )
    }
}

/// secp256k1 is an elliptic curve whose underlying group has a cofactor of `h = 1`. There are up
/// to `2 * (h + 1) = 4` valid points for a given signature.
pub enum DerivedSecp256k1Points {
    Pair([Secp256k1Point; 2]),
    Quartet([Secp256k1Point; 4]),
}

impl DerivedSecp256k1Points {
    /// Returns an iterator over the secp256k1 points.
    pub fn iter(&self) -> impl Iterator<Item = &Secp256k1Point> {
        match self {
            Self::Pair(points) => points.iter(),
            Self::Quartet(points) => points.iter(),
        }
    }
}
