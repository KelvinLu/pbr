use programming_bitcoin_in_rust::*;

use crypto::ecdsa::signature::Signature;
use crypto::digest::hash_256;
use crypto::secp256k1::Secp256k1Point;
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

    assert!(signature.verify_point_secp256k1(z, point));

    let derived_points = signature.derive_points_secp256k1(z).unwrap();

    let mut original_point_found = false;

    for derived_point in derived_points.iter() {
        assert!(signature.verify_point_secp256k1(z, *derived_point));

        if *derived_point == point { original_point_found = true }
    }

    assert!(original_point_found);

    assert!(signature.verify_by_derivation_secp256k1(z));
}
