use programming_bitcoin_in_rust::*;

use crypto::ecdsa::signature::Signature;
use crypto::digest::hash_256;
use crypto::secp256k1::Secp256k1Point;
use util::number::U256;
use util::number::uint;

pub fn run() {
    uint! {
        let point = Secp256k1Point::new(
            0x887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c_U256,
            0x61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34_U256
        );

        let z = 0xec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60_U256;
        let r = 0xac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395_U256;
        let s = 0x68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4_U256;
    }

    let signature = Signature { r: r, s: s };

    assert!(signature.verify_point_secp256k1(z, point));

    uint! {
        let z = 0x7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d_U256;
        let r = 0xeff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c_U256;
        let s = 0xc7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6_U256;
    }

    let signature = Signature { r: r, s: s };

    assert!(signature.verify_point_secp256k1(z, point));

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
}
