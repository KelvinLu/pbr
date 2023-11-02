use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::finite_field_element::FiniteFieldElement;

pub fn exercise() {
    prompt("[test] Finite field elements (multiplication)");

    let a = FiniteFieldElement::from((24, 31_u32));
    let b = FiniteFieldElement::from((19, 31_u32));
    let result = a.clone() * b.clone();
    let expected = FiniteFieldElement::from((22, 31_u32));

    assert_eq!(expected, result);

    message(&format!("{} * {} = {}", &a, &b, &result));
}
