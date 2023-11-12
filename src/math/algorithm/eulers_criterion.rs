//! Euler's criterion.

use crate::util::number::U256;

/// Determines if a given integer `n` is a quadratic residue, modulo a prime `p`.
///
/// When `p` is an odd prime and `n` is an integer coprime to `p` ...
///
/// ```
/// n ^ ((p - 1) / 2)
/// ```
///
/// ... is congruent with `1 (mod p)` or `-1 (mod p)`. The former result indicates that `n` is a
/// quadratic residue modulo `p`, and the latter otherwise.
///
/// A quadratic residue indicates that `x^2` exists in congruence with `n (mod p)`.
pub fn quadratic_residue(n: U256, p: U256) -> bool {
    n.pow_mod((p - U256::from(1)) / U256::from(2), p) == U256::from(1)
}
