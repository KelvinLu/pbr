use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::elliptic_curve::point::EllipticCurvePoint;
use math::finite_field_element::FiniteFieldElement;

pub fn exercise() {
    prompt("[test] Elliptic curve (point addition)");

    let a = FiniteFieldElement::from((0, 223_u32));
    let b = FiniteFieldElement::from((7, 223_u32));

    let p1 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((192, 223_u32)), FiniteFieldElement::from((105, 223_u32))), (a, b))).unwrap();
    let p2 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((17, 223_u32)), FiniteFieldElement::from((56, 223_u32))), (a, b))).unwrap();
    let result = p1.clone() + p2.clone();
    let expected = EllipticCurvePoint::try_from(((FiniteFieldElement::from((170, 223_u32)), FiniteFieldElement::from((142, 223_u32))), (a, b))).unwrap();

    assert_eq!(expected, result);

    message(&format!("({}) + ({}) = ({})", &p1, &p2, &result));

    let p1 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((47, 223_u32)), FiniteFieldElement::from((71, 223_u32))), (a, b))).unwrap();
    let p2 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((117, 223_u32)), FiniteFieldElement::from((141, 223_u32))), (a, b))).unwrap();
    let result = p1.clone() + p2.clone();
    let expected = EllipticCurvePoint::try_from(((FiniteFieldElement::from((60, 223_u32)), FiniteFieldElement::from((139, 223_u32))), (a, b))).unwrap();

    assert_eq!(expected, result);

    message(&format!("({}) + ({}) = ({})", &p1, &p2, &result));

    let p1 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((143, 223_u32)), FiniteFieldElement::from((98, 223_u32))), (a, b))).unwrap();
    let p2 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((76, 223_u32)), FiniteFieldElement::from((66, 223_u32))), (a, b))).unwrap();
    let result = p1.clone() + p2.clone();
    let expected = EllipticCurvePoint::try_from(((FiniteFieldElement::from((47, 223_u32)), FiniteFieldElement::from((71, 223_u32))), (a, b))).unwrap();

    assert_eq!(expected, result);

    message(&format!("({}) + ({}) = ({})", &p1, &p2, &result));

    let p1 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((170, 223_u32)), FiniteFieldElement::from((142, 223_u32))), (a, b))).unwrap();
    let p2 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((60, 223_u32)), FiniteFieldElement::from((139, 223_u32))), (a, b))).unwrap();
    let result = p1.clone() + p2.clone();
    let expected = EllipticCurvePoint::try_from(((FiniteFieldElement::from((220, 223_u32)), FiniteFieldElement::from((181, 223_u32))), (a, b))).unwrap();

    assert_eq!(expected, result);

    message(&format!("({}) + ({}) = ({})", &p1, &p2, &result));

    let p1 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((47, 223_u32)), FiniteFieldElement::from((71, 223_u32))), (a, b))).unwrap();
    let p2 = EllipticCurvePoint::try_from(((FiniteFieldElement::from((17, 223_u32)), FiniteFieldElement::from((56, 223_u32))), (a, b))).unwrap();
    let result = p1.clone() + p2.clone();
    let expected = EllipticCurvePoint::try_from(((FiniteFieldElement::from((215, 223_u32)), FiniteFieldElement::from((68, 223_u32))), (a, b))).unwrap();

    assert_eq!(expected, result);

    message(&format!("({}) + ({}) = ({})", &p1, &p2, &result));
}
