use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::elliptic_curve::point::EllipticCurvePoint;

pub fn exercise() {
    prompt("Elliptic curve (y ^ 2) = (x ^ 3) + (5 * x) + 7");

    let a = 5;
    let b = 7;

    for coordinates in [(2, 4), (-1, -1), (18, 77), (5, 7)] {
        let (x, y) = coordinates;

        section(&format!("({}, {})", x, y));

        if let Ok(point) = EllipticCurvePoint::try_from(((x, y), (a, b))) {
            show_display(&point);
        } else {
            message("Not on curve.");
        }
    }
}
