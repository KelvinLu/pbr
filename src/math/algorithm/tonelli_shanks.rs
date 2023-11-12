//! Tonelli-Shanks algorithm.

use crate::util::number::U256;

/// Error when the given prime number is incongruent with some other number within some algebraic
/// structure.
#[derive(Debug)]
pub struct IncongruentPrimeErr;

/// Returns the square roots of some non-zero `n`, modulo `p > 2`.
///
/// `p` itself must be congruent to `3 (mod 4)`, or an `IncongruentPrimeErr` will be returned
/// instead.
pub fn square_roots(n: U256, p: U256) -> Result<(U256, U256), IncongruentPrimeErr> {
    if (p % U256::from(4)) != U256::from(3) {
        return Err(IncongruentPrimeErr);
    }

    let r = n.pow_mod((p + U256::from(1)) / U256::from(4), p);

    Ok((r, p - r))
}
