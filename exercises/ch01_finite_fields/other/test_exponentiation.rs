use programming_bitcoin_in_rust::*;

use math::finite_field_element::FiniteFieldElement;

pub fn run() {
    let a = FiniteFieldElement::from((17, 31_u32));
    let result = a.pow_u32(3);
    let expected = FiniteFieldElement::from((15, 31_u32));

    assert_eq!(expected, result);

    let a = FiniteFieldElement::from((5, 31_u32));
    let b = FiniteFieldElement::from((18, 31_u32));
    let result = a.pow_u32(5) * b;
    let expected = FiniteFieldElement::from((16, 31_u32));

    assert_eq!(expected, result);
}
