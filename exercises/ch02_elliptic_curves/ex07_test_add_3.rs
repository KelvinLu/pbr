use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::elliptic_curve::point::EllipticCurvePoint;

pub fn exercise() {
    prompt("[test] Elliptic curve points (point addition, on the same point)");

    section("Adding a point to itself produces another point reflected upon the intercept on the elliptic curve");

    let a = EllipticCurvePoint::try_from(((-1, -1), (5, 7))).unwrap();
    let b = EllipticCurvePoint::try_from(((18, 77), (5, 7))).unwrap();

    message(&format!("({}) + ({}) = ({})", a, a, b));

    assert_eq!(a + a, b);

    section("Adding a point to itself at 'y = 0' produces a vertical tangent intercepting the point at infinity");

    let a = EllipticCurvePoint::try_from(((-2, 0), (2, 12))).unwrap();

    message(&format!("({}) + ({}) = ({})", a, a, EllipticCurvePoint::PointAtInfinity));

    assert_eq!(a + a, EllipticCurvePoint::PointAtInfinity);
}
