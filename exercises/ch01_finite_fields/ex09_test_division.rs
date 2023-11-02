use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::finite_field_element::FiniteFieldElement;

pub fn exercise() {
    prompt("[test] Finite field elements (division)");

    let a = FiniteFieldElement::from((3, 31_u32));
    let b = FiniteFieldElement::from((24, 31_u32));
    let result = a.clone() / b.clone();
    let expected = FiniteFieldElement::from((4, 31_u32));

    assert_eq!(expected, result);

    message(&format!("{} / {} = {}", &a, &b, &result));

    let a = FiniteFieldElement::from((17, 31_u32));
    let result = a.clone().pow_i32(-3);
    let expected = FiniteFieldElement::from((29, 31_u32));

    assert_eq!(expected, result);

    message(&format!("({} ^ -3) = {}", &a, &result));

    let a = FiniteFieldElement::from((4, 31_u32));
    let b = FiniteFieldElement::from((11, 31_u32));
    let result = a.clone().pow_i32(-4) * b.clone();
    let expected = FiniteFieldElement::from((13, 31_u32));

    assert_eq!(expected, result);

    message(&format!("({} ^ -4) * {} = {}", &a, &b, &result));
}
