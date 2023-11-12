use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::elliptic_curve::point::EllipticCurvePoint;

pub fn exercise() {
    prompt("Elliptic curve (y ^ 2) = (x ^ 3) + (5 * x) + 7");

    let a = EllipticCurvePoint::try_from(((2, 5), (5, 7))).unwrap();
    let b = EllipticCurvePoint::try_from(((-1, -1), (5, 7))).unwrap();
    let exp = a + b;

    message(&format!("({}) + ({}) = ({})", a, b, exp));
}
