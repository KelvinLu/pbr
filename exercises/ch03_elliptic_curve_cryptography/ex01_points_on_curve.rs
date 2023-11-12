use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::elliptic_curve::point::EllipticCurvePoint;
use math::finite_field_element::FiniteFieldElement;

pub fn exercise() {
    prompt("Elliptic curve (y ^ 2) = (x ^ 3) + 7, over finite field of prime order 223");

    let a = FiniteFieldElement::from((0, 223_u32));
    let b = FiniteFieldElement::from((7, 223_u32));

    let points: Vec<(FiniteFieldElement, FiniteFieldElement)> = [(192, 105), (17, 56), (200, 119), (1, 193), (42, 99)]
        .iter()
        .map(|(x, y)| (FiniteFieldElement::from((*x, 223_u32)), FiniteFieldElement::from((*y, 223_u32))))
        .collect();

    for coordinates in points {
        let (x, y) = coordinates;

        section(&format!("({}, {})", x, y));

        if let Ok(point) = EllipticCurvePoint::try_from(((x, y), (a, b))) {
            show_display(&point);
        } else {
            message("Not on curve.");
        }
    }

    let p1 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((192, 223_u32)), FiniteFieldElement::from((105, 223_u32))), (a, b))).unwrap();
    let p2 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((17, 223_u32)), FiniteFieldElement::from((56, 223_u32))), (a, b))).unwrap();

    section(&format!("({}) + ({}) =", p1, p2));
    show_display(&(p1 + p2));
}
