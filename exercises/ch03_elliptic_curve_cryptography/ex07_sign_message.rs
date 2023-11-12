use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use crypto::ecdsa::signature::Signature;
use crypto::secp256k1::Secp256k1Point;
use crypto::digest::hash_256;
use util::number::U256;

pub fn exercise() {
    prompt("ECDSA with secp256k1 (message signing)");

    let z = U256::from_be_bytes(hash_256(b"Programming Bitcoin!"));
    let e = U256::from(12345);
    let k = U256::from(1234567890);

    section("Signature creation");

    let signature = Signature::new_secp256k1(z, e, k);

    message(&format!("r: {:?}", signature.r));
    message(&format!("s: {:?}", signature.s));

    let point = e * Secp256k1Point::generator_point();

    section(&format!("Signature verified: {}", signature.verify_point_secp256k1(z, point)));
    message(&format!("Data: {:?}", z));
    message(&format!("Point: {:#?}", point));
}
