use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::finite_field_element::FiniteFieldElement;

pub fn exercise() {
    prompt("[test] Finite field elements (subtraction)");

    let a = FiniteFieldElement::from((29, 31_u32));
    let b = FiniteFieldElement::from((4, 31_u32));
    let result = a.clone() - b.clone();
    let expected = FiniteFieldElement::from((25, 31_u32));

    assert_eq!(expected, result);

    message(&format!("{} - {} = {}", &a, &b, &result));

    let a = FiniteFieldElement::from((15, 31_u32));
    let b = FiniteFieldElement::from((30, 31_u32));
    let result = a.clone() - b.clone();
    let expected = FiniteFieldElement::from((16, 31_u32));

    assert_eq!(expected, result);

    message(&format!("{} - {} = {}", &a, &b, &result));
}
