use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::elliptic_curve::point::EllipticCurvePoint;
use math::finite_field_element::FiniteFieldElement;

pub fn exercise() {
    prompt("Elliptic curve (y ^ 2) = (x ^ 3) + 7, over finite field of prime order 223");

    let a = FiniteFieldElement::from((0, 223_u32));
    let b = FiniteFieldElement::from((7, 223_u32));

    let n = 2;

    let x = FiniteFieldElement::from((192, 223_u32));
    let y = FiniteFieldElement::from((105, 223_u32));
    let point = EllipticCurvePoint::try_from(((x, y), (a, b))).unwrap();

    section(&format!("{} * ({}) =", n, point));
    show_display(&(n * point));

    let x = FiniteFieldElement::from((143, 223_u32));
    let y = FiniteFieldElement::from((98, 223_u32));
    let point = EllipticCurvePoint::try_from(((x, y), (a, b))).unwrap();

    section(&format!("{} * ({}) =", n, point));
    show_display(&(n * point));

    let x = FiniteFieldElement::from((47, 223_u32));
    let y = FiniteFieldElement::from((71, 223_u32));
    let point = EllipticCurvePoint::try_from(((x, y), (a, b))).unwrap();

    section(&format!("{} * ({}) =", n, point));
    show_display(&(n * point));

    for n in 19..=23 {
        section(&format!("{} * ({}) =", n, point));
        show_display(&(n * point));
    }
}
