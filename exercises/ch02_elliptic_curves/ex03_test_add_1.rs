use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::elliptic_curve::point::EllipticCurvePoint;

pub fn exercise() {
    prompt("[test] Elliptic curve points (point addition, with point at infinity)");

    let a = EllipticCurvePoint::PointAtInfinity;
    let b = EllipticCurvePoint::try_from(((2, 5), (5, 7))).unwrap();
    let c = EllipticCurvePoint::try_from(((2, -5), (5, 7))).unwrap();

    section("The point at infinity serves as the additive identitity for point addition");

    message(&format!("({}) + ({}) = ({})", a, b, b));
    message(&format!("({}) + ({}) = ({})", b, a, b));

    assert_eq!(a + b, b);
    assert_eq!(b + a, b);

    section("Adding the two points at some shared 'x' coordinate produces the infinity point");

    message(&format!("({}) + ({}) = ({})", b, c, a));

    assert_eq!(b + c, a);
}
