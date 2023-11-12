use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use crypto::ecdsa::signature::Signature;
use crypto::digest::hash_256;
use crypto::secp256k1::{Secp256k1Point, ELLIPTIC_CURVE_ORDER};
use math::elliptic_curve::point::EllipticCurvePoint;
use math::number::Number;
use util::number::U256;
use util::number::Uint;
use util::number::uint;

type U513 = Uint<513, 9>;

pub fn exercise() {
    prompt("ECDSA with secp256k1 (signature verification)");

    uint! {
        let point = Secp256k1Point::new(
            0x887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c_U256,
            0x61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34_U256
        );
    }

    section("Signature one");

    uint! {
        let z = 0xec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60_U256;
        let r = 0xac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395_U256;
        let s = 0x68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4_U256;
    }

    let signature = Signature { r: r, s: s };

    message("Could be verified:");
    show_display(&signature.verify_point_secp256k1(z, point));

    section("Signature two");

    uint! {
        let z = 0x7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d_U256;
        let r = 0xeff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c_U256;
        let s = 0xc7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6_U256;
    }

    let signature = Signature { r: r, s: s };

    message("Could be verified:");
    show_display(&signature.verify_point_secp256k1(z, point));

    section("ECDSA: 'r' and 's' are signature values, produced for some data 'z' and verifiable against a public point (generated from a secret 'e' and randomly committed 'k')");

    let secret_as_string = b"my secret";
    let secret_digest_bytes = hash_256(secret_as_string);
    let e = U256::from_be_bytes(secret_digest_bytes);

    message("SHA-256(SHA-256('my secret'))");
    message("e =");
    show_debug(&e);

    let point = e * Secp256k1Point::generator_point();

    message("e * G = <public point> =");
    show_display(&point);

    let message_as_string = b"my message";
    let message_digest_bytes = hash_256(message_as_string);
    let z = U256::from_be_bytes(message_digest_bytes);

    section("Message signing");

    message("SHA-256(SHA-256('my message'))");
    message("z =");
    show_debug(&z);

    let k = U256::from(1234567890);

    message("k =");
    show_debug(&k);

    let EllipticCurvePoint::PointOnCurve(r_point) = (k * Secp256k1Point::generator_point()).into() else {
        panic!("the calculation should not result with point at infinity")
    };
    let Number::FiniteFieldElement(r) = r_point.x else { panic!("expected a finite field element") };
    let r = r.value;

    message("k * G =");
    show_display(&r_point);

    let k_inv = k.pow_mod(ELLIPTIC_CURVE_ORDER - U256::from(2), ELLIPTIC_CURVE_ORDER);

    message("k ^ -1 mod n =");
    show_debug(&k_inv);

    let mut intermediate = U513::from(r);
    intermediate *= U513::from(e);

    message("r * e =");
    show_debug(&intermediate);

    intermediate += U513::from(z);

    message("z + (r * e) =");
    show_debug(&intermediate);

    section("Produced signature values");

    let Number::FiniteFieldElement(r) = r_point.x else { panic!("expected a finite field element") };
    let r = r.value;

    message("r = <'x' coordinate of (k * G)> =");
    show_debug(&r);

    let s = intermediate.mul_mod(U513::from(k_inv), U513::from(ELLIPTIC_CURVE_ORDER));
    let s = U256::from(s);

    message("s = (z + r * e) / k mod n =");
    show_debug(&s);
}
