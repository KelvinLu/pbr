//! RFC 6979: Deterministic usage of DSA and ECDSA.

use crate::crypto::digest::{Hmac, Mac, Sha256};
use crate::util::number::U256;

/// Produces a deterministic `k` (random commitment) value given the `data` to sign along with the
/// (private key) `secret_e` used.
///
/// Uses HMAC SHA-256 digests, which also assume the `data` to be signed has also undergone similar
/// 256 bit digests (for uniformity under a normal distribution).
pub fn deterministic_k(data: U256, secret_e: U256, elliptic_curve_order: U256) -> U256 {
    let mut k: [u8; 32] = [0; 32];
    let mut v: [u8; 32] = [1; 32];

    let mut data = data;
    let mut hmac: Hmac::<Sha256>;

    while data > elliptic_curve_order {
        data -= elliptic_curve_order;
    }

    let data: [u8; 32] = data.to_be_bytes();
    let secret_e: [u8; 32] = secret_e.to_be_bytes();

    hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
    hmac.update(&v);
    hmac.update(&[0_u8]);
    hmac.update(&secret_e);
    hmac.update(&data);

    k = hmac.finalize().into_bytes().into();

    hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
    hmac.update(&v);

    v = hmac.finalize().into_bytes().into();

    hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
    hmac.update(&v);
    hmac.update(&[1_u8]);
    hmac.update(&secret_e);
    hmac.update(&data);

    k = hmac.finalize().into_bytes().into();

    hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
    hmac.update(&v);

    v = hmac.finalize().into_bytes().into();

    loop {
        hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
        hmac.update(&v);

        v = hmac.finalize().into_bytes().into();

        let candidate = U256::from_be_bytes(v.into());

        if candidate > U256::ZERO && candidate < elliptic_curve_order {
            return candidate;
        }

        hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
        hmac.update(&v);
        hmac.update(&[0_u8]);

        k = hmac.finalize().into_bytes().into();

        hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
        hmac.update(&v);

        v = hmac.finalize().into_bytes().into();
    }
}
