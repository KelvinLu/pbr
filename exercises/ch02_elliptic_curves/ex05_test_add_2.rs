use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::elliptic_curve::point::EllipticCurvePoint;

pub fn exercise() {
    prompt("[test] Elliptic curve points (point addition, slope calculation)");

    section("Adding two points generally produces another point on the elliptic curve");

    let a = EllipticCurvePoint::try_from(((3, 7), (5, 7))).unwrap();
    let b = EllipticCurvePoint::try_from(((-1, -1), (5, 7))).unwrap();
    let c = EllipticCurvePoint::try_from(((2, -5), (5, 7))).unwrap();

    message(&format!("({}) + ({}) = ({})", a, b, c));

    assert_eq!(a + b, c);
}
