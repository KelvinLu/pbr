use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::elliptic_curve::point::EllipticCurvePoint;

pub fn exercise() {
    prompt("[test] Elliptic curve points (not equals)");

    let a = EllipticCurvePoint::try_from(((3, -7), (5, 7))).unwrap();
    let b = EllipticCurvePoint::try_from(((18, 77), (5, 7))).unwrap();

    section("Assert a != b");

    message(&format!("a = {}", &a));
    message(&format!("b = {}", &b));

    assert_ne!(a, b);

    section("Assert a == a");

    assert_eq!(a, a);
}
