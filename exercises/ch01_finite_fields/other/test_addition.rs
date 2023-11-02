use programming_bitcoin_in_rust::*;

use math::finite_field_element::FiniteFieldElement;

pub fn run() {
    let a = FiniteFieldElement::from((2, 31_u32));
    let b = FiniteFieldElement::from((15, 31_u32));
    let result = a + b;
    let expected = FiniteFieldElement::from((17, 31_u32));

    assert_eq!(expected, result);

    let a = FiniteFieldElement::from((17, 31_u32));
    let b = FiniteFieldElement::from((21, 31_u32));
    let result = a + b;
    let expected = FiniteFieldElement::from((7, 31_u32));

    assert_eq!(expected, result);
}
