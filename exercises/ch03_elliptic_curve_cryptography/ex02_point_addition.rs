use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::elliptic_curve::point::EllipticCurvePoint;
use math::finite_field_element::FiniteFieldElement;

pub fn exercise() {
    prompt("Elliptic curve (y ^ 2) = (x ^ 3) + 7, over finite field of prime order 223");

    let a = FiniteFieldElement::from((0, 223_u32));
    let b = FiniteFieldElement::from((7, 223_u32));

    let p1 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((170, 223_u32)), FiniteFieldElement::from((142, 223_u32))), (a, b))).unwrap();
    let p2 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((60, 223_u32)), FiniteFieldElement::from((139, 223_u32))), (a, b))).unwrap();

    section(&format!("({}) + ({}) =", p1, p2));
    show_display(&(p1 + p2));

    let p1 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((47, 223_u32)), FiniteFieldElement::from((71, 223_u32))), (a, b))).unwrap();
    let p2 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((17, 223_u32)), FiniteFieldElement::from((56, 223_u32))), (a, b))).unwrap();

    section(&format!("({}) + ({}) =", p1, p2));
    show_display(&(p1 + p2));

    let p1 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((143, 223_u32)), FiniteFieldElement::from((98, 223_u32))), (a, b))).unwrap();
    let p2 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((76, 223_u32)), FiniteFieldElement::from((66, 223_u32))), (a, b))).unwrap();

    section(&format!("({}) + ({}) =", p1, p2));
    show_display(&(p1 + p2));
}
