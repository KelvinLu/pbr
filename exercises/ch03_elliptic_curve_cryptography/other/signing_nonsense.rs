use programming_bitcoin_in_rust::*;

use crypto::ecdsa::signature::Signature;
use crypto::digest::hash_256;
use crypto::secp256k1::{Secp256k1Point, ELLIPTIC_CURVE_ORDER};
use math::elliptic_curve::point::EllipticCurvePoint;
use math::finite_field_element::FiniteFieldElement;
use math::number::Number;
use util::number::U256;
use util::number::uint;

pub fn run() {
    uint! {
        let z = U256::from_be_bytes(hash_256(b"Programming Bitcoin!"));
        let e = 12345_U256;
        let k = 1234567890_U256;
    }

    let point = e * Secp256k1Point::generator_point();
    let signature = Signature::new_secp256k1(z, e, k);

    uint! {
        let r = 0x2b698a0f0a4041b77e63488ad48c23e8e8838dd1fb7520408b121697b782ef22_U256;
        let s = 0x1dbc63bfef4416705e602a7b564161167076d8b20990a0f26f316cff2cb0bc1a_U256;
    }

    assert_eq!(r, signature.r);
    assert_eq!(s, signature.s);
    assert!(signature.verify_point_secp256k1(z, point));

    // Choose random values.
    uint! {
        let u = 0x0000000000000000000000000000000000000000000000000000000000000420_U256;
        let v = 0x6900000000000000000000000000000000000000000000000000000000000000_U256;
    }

    assert!(u < ELLIPTIC_CURVE_ORDER);
    assert!(v < ELLIPTIC_CURVE_ORDER);

    let Secp256k1Point::Point(random_r_point) = (u * Secp256k1Point::generator_point()) + (v * point);

    let (new_r_value, new_s_value, random_data) = {
        let EllipticCurvePoint::PointOnCurve(random_r_point) = random_r_point else {
            panic!("encountered point at infinity");
        };

        let Number::FiniteFieldElement(random_r) = random_r_point.x else {
            panic!("expected a finite field element")
        };

        let random_r = random_r.value % ELLIPTIC_CURVE_ORDER;

        let random_s = random_r * FiniteFieldElement::new(v, ELLIPTIC_CURVE_ORDER).modular_inverse().unwrap();

        let random_z = u * random_s;

        (random_r, random_s.value, random_z.value)
    };

    // These values are dependent on the choices of u, v.
    uint! {
        // The original point also verifies this nonsense value of z ...
        let random_z = 0xc394adff04a4acc0385388b39297abdb817a1149902e293de0f8d6c6ad8b933d_U256;

        // ... with these signature parameters.
        let random_r = 0x0a39027cbaf359f28f374962b083160a414ae08c39c41a43ac10692a3d2e2c64_U256;
        let random_s = 0x6d84bf31f03f2fadc1fe23c40c80a0e34d2f67163a451f2384138911a9169386_U256;
    }

    assert_eq!(random_data, random_z);
    assert_ne!(random_data, z);

    assert_eq!(new_r_value, random_r);
    assert_eq!(new_s_value, random_s);

    let new_signature = Signature { r: new_r_value, s: new_s_value };

    assert!(new_signature.verify_point_secp256k1(random_data, point));
}
