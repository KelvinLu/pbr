use programming_bitcoin_in_rust::*;

use crypto::ecdsa::signature::Signature;
use crypto::digest::hash_256;
use crypto::secp256k1::{Secp256k1Point, ELLIPTIC_CURVE_ORDER};
use math::finite_field_element::FiniteFieldElement;
use util::number::U256;
use util::number::uint;

pub fn run() {
    uint! {
        let e = 12345_U256;
        let k = 1234567890_U256;

        let z_1 = U256::from_be_bytes(hash_256(b"Reusing the random commitment 'k' ..."));
        let z_2 = U256::from_be_bytes(hash_256(b"... reveals the secret 'e'!"));
    }

    let point = e * Secp256k1Point::generator_point();

    let signature_1 = Signature::new_secp256k1(z_1, e, k);
    let signature_2 = Signature::new_secp256k1(z_2, e, k);

    assert!(signature_1.verify_point_secp256k1(z_1, point));
    assert!(signature_2.verify_point_secp256k1(z_2, point));

    // k * G generates the same R point and r-value.
    assert_eq!(signature_1.r, signature_2.r);

    let r = signature_1.r;

    let s_1 = signature_1.s;
    let s_2 = signature_2.s;

    // Letting s' = s^-1 mod n also produces valid signatures for the same data,
    // albeit at different points.
    let s_3 = s_1.inv_mod(ELLIPTIC_CURVE_ORDER).unwrap();
    let s_4 = s_2.inv_mod(ELLIPTIC_CURVE_ORDER).unwrap();

    let signature_3 = Signature { r: r, s: s_3 };
    let signature_4 = Signature { r: r, s: s_4 };

    for p in signature_3.derive_points_secp256k1(z_1).unwrap().iter() {
        assert!(signature_3.verify_point_secp256k1(z_1, *p));
    }

    for p in signature_4.derive_points_secp256k1(z_2).unwrap().iter() {
        assert!(signature_4.verify_point_secp256k1(z_2, *p));
    }

    // There are four candidates to consider for deriving k --
    // let z and z' be the two messages, where s, s' are the s-values for the
    // two corresponding signatures.
    //
    // (z - z') / (s - s')
    // (z - z') / (s + s')
    // (z - z') / (-s - s')
    // (z - z') / (-s + s')
    let intermediate = FiniteFieldElement::new(z_1, ELLIPTIC_CURVE_ORDER) - FiniteFieldElement::new(z_2, ELLIPTIC_CURVE_ORDER);

    let candidate_1 = FiniteFieldElement::new(s_1, ELLIPTIC_CURVE_ORDER) - FiniteFieldElement::new(s_2, ELLIPTIC_CURVE_ORDER);
    let candidate_2 = FiniteFieldElement::new(s_1, ELLIPTIC_CURVE_ORDER) + FiniteFieldElement::new(s_2, ELLIPTIC_CURVE_ORDER);
    let candidate_3 = (-FiniteFieldElement::new(s_1, ELLIPTIC_CURVE_ORDER)) - FiniteFieldElement::new(s_2, ELLIPTIC_CURVE_ORDER);
    let candidate_4 = (-FiniteFieldElement::new(s_1, ELLIPTIC_CURVE_ORDER)) + FiniteFieldElement::new(s_2, ELLIPTIC_CURVE_ORDER);

    let k_1 = intermediate / candidate_1;
    let k_2 = intermediate / candidate_2;
    let k_3 = intermediate / candidate_3;
    let k_4 = intermediate / candidate_4;

    // Test the possible values of k against either of the two original
    // signatures with their signed message (and against the original k itself).
    let r = FiniteFieldElement::new(r, ELLIPTIC_CURVE_ORDER);

    let z = FiniteFieldElement::new(z_1, ELLIPTIC_CURVE_ORDER);
    let s = FiniteFieldElement::new(s_1, ELLIPTIC_CURVE_ORDER);

    let mut e_found = false;
    let mut k_found = false;

    for candidate_k in [k_1, k_2, k_3, k_4] {
        let candidate_e = ((s * candidate_k) - z) / r;

        if e == candidate_e.value { e_found = true }
        if k == candidate_k.value { k_found = true }
    }

    assert!(k_found);
    assert!(e_found);

    let mut e_found = false;
    let mut k_found = false;

    let z = FiniteFieldElement::new(z_2, ELLIPTIC_CURVE_ORDER);
    let s = FiniteFieldElement::new(s_2, ELLIPTIC_CURVE_ORDER);

    for candidate_k in [k_1, k_2, k_3, k_4] {
        let candidate_e = ((s * candidate_k) - z) / r;

        if e == candidate_e.value { e_found = true }
        if k == candidate_k.value { k_found = true }
    }

    assert!(k_found);
    assert!(e_found);
}
