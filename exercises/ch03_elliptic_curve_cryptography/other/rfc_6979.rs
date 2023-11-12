use programming_bitcoin_in_rust::*;

use crypto::ecdsa::signature::Signature;
use crypto::ecdsa::rfc_6979::deterministic_k;
use crypto::digest::hash_256;
use crypto::secp256k1::ELLIPTIC_CURVE_ORDER;
use math::finite_field_element::FiniteFieldElement;
use util::number::U256;
use util::number::uint;

pub fn run() {
    uint! {
        let e = 12345_U256;

        let z = U256::from_be_bytes(hash_256(b"Programming Bitcoin!"));
        let k = 0xabef7a40d9bd76aef7ee7e733404ecfcd8041550a68625d7cc0608b0025038b1_U256;

        let z_1 = U256::from_be_bytes(hash_256(b"We will use a deterministic value of k ..."));
        let z_2 = U256::from_be_bytes(hash_256(b"... as described by RFC 6979"));
    }

    assert_eq!(k, deterministic_k(z, e, U256::from(ELLIPTIC_CURVE_ORDER)));

    let signature_1 = Signature::sign_secp256k1(z_1, e);
    let signature_2 = Signature::sign_secp256k1(z_2, e);

    assert_ne!(signature_1.r, signature_2.r);

    let r = signature_1.r;

    let s_1 = signature_1.s;
    let s_2 = signature_2.s;

    let intermediate = FiniteFieldElement::new(z_1, ELLIPTIC_CURVE_ORDER) - FiniteFieldElement::new(z_2, ELLIPTIC_CURVE_ORDER);

    let candidate_1 = FiniteFieldElement::new(s_1, ELLIPTIC_CURVE_ORDER) - FiniteFieldElement::new(s_2, ELLIPTIC_CURVE_ORDER);
    let candidate_2 = FiniteFieldElement::new(s_1, ELLIPTIC_CURVE_ORDER) + FiniteFieldElement::new(s_2, ELLIPTIC_CURVE_ORDER);
    let candidate_3 = (-FiniteFieldElement::new(s_1, ELLIPTIC_CURVE_ORDER)) - FiniteFieldElement::new(s_2, ELLIPTIC_CURVE_ORDER);
    let candidate_4 = (-FiniteFieldElement::new(s_1, ELLIPTIC_CURVE_ORDER)) + FiniteFieldElement::new(s_2, ELLIPTIC_CURVE_ORDER);

    let k_1 = intermediate / candidate_1;
    let k_2 = intermediate / candidate_2;
    let k_3 = intermediate / candidate_3;
    let k_4 = intermediate / candidate_4;

    let r = FiniteFieldElement::new(r, ELLIPTIC_CURVE_ORDER);

    let z = FiniteFieldElement::new(z_1, ELLIPTIC_CURVE_ORDER);
    let s = FiniteFieldElement::new(s_1, ELLIPTIC_CURVE_ORDER);

    let mut e_found = false;

    for candidate_k in [k_1, k_2, k_3, k_4] {
        let candidate_e = ((s * candidate_k) - z) / r;

        if e == candidate_e.value { e_found = true }
    }

    assert!(e_found == false);

    let mut e_found = false;

    let z = FiniteFieldElement::new(z_2, ELLIPTIC_CURVE_ORDER);
    let s = FiniteFieldElement::new(s_2, ELLIPTIC_CURVE_ORDER);

    for candidate_k in [k_1, k_2, k_3, k_4] {
        let candidate_e = ((s * candidate_k) - z) / r;

        if e == candidate_e.value { e_found = true }
    }

    assert!(e_found == false);
}
