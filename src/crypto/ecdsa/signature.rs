//! ECDSA signatures.

use crate::math::elliptic_curve::point::EllipticCurvePoint;
use crate::math::number::Number;
use crate::crypto::ecdsa::rfc_6979::deterministic_k;
use crate::util::number::U256;
use crate::util::number::Uint;

type U513 = Uint<513, 9>;

/// An ECDSA signature.
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub struct Signature {
    /// r-value.
    pub r: U256,

    /// s-value.
    pub s: U256,
}

/// ECDSA signature operations.
impl Signature {
    /// Produce a signature for some `data` value, signed using a (private key) `secret_e`.
    ///
    /// `data` and `secret_e` may be expressed as 256 bit arguments (32 bytes).
    ///
    /// This relies on an underlying elliptic curve and a generator point.
    ///
    /// The corresponding (public key) point is `secret_e * generator_point`, which may be used to
    /// verify this signature against the original `data`.
    ///
    /// The random commitment is chosen in a deterministic manner, based on RFC 6979. It utilizes
    /// HMAC digests (SHA-256) against the given `data` and `secret_e`, and assumes the `data` has
    /// already been hashed through a similar 256 bit digest.
    pub fn sign(
        data: U256,
        secret_e: U256,
        generator_point: EllipticCurvePoint,
        elliptic_curve_order: U256
    ) -> Self {
        let k = deterministic_k(data, secret_e, elliptic_curve_order);

        Self::new(
            data,
            secret_e,
            k,
            generator_point,
            elliptic_curve_order
        )
    }

    /// Produce a signature for some `data` value, signed using a (private key) `secret_e` against
    /// a random commitment (`random_commitment`).
    ///
    /// `data`, `secret_e`, and `random_commitment` may be expressed as 256 bit arguments (32 bytes).
    ///
    /// This relies on an underlying elliptic curve and a generator point.
    ///
    /// The corresponding (public key) point is `secret_e * generator_point`, which may be used to
    /// verify this signature against the original `data`.
    ///
    /// The signature is given a lower-half s-value to respect BIP-146.
    pub fn new(
        data: U256,
        secret_e: U256,
        random_commitment: U256,
        generator_point: EllipticCurvePoint,
        elliptic_curve_order: U256
    ) -> Self {
        let r_point = random_commitment * generator_point;
        let EllipticCurvePoint::PointOnCurve(r_point) = r_point else { panic!("the calculation should not result with point at infinity") };
        let Number::FiniteFieldElement(r) = r_point.x else { panic!("expected a finite field element") };

        let k_inv = random_commitment.pow_mod(elliptic_curve_order - U256::from(2), elliptic_curve_order);

        let r = r.value;

        let mut intermediate = U513::from(r);
        intermediate *= U513::from(secret_e);
        intermediate += U513::from(data);

        let s = intermediate.mul_mod(U513::from(k_inv), U513::from(elliptic_curve_order));
        let mut s = U256::from(s);

        // Optimize for a low s-value
        if s > (elliptic_curve_order / U256::from(2)) {
            s = elliptic_curve_order - s;
        }

        Self {
            r: r,
            s: s,
        }
    }

    /// Verify the signature against some `data` value, given an elliptic curve `point`.
    pub fn verify_point(
        &self,
        data: U256,
        point: EllipticCurvePoint,
        generator_point: EllipticCurvePoint,
        elliptic_curve_order: U256
    ) -> bool {
        let s_inv = self.s.pow_mod(elliptic_curve_order - U256::from(2), elliptic_curve_order);

        let u = data.mul_mod(s_inv, elliptic_curve_order);
        let v = self.r.mul_mod(s_inv, elliptic_curve_order);

        match (u * generator_point) + (v * point) {
            EllipticCurvePoint::PointOnCurve(point) => {
                let Number::FiniteFieldElement(x) = point.x else { panic!("expected a finite field element") };

                self.r == x.value
            },
            _ => false
        }
    }
}
