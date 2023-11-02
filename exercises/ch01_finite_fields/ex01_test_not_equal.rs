use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::finite_field_element::FiniteFieldElement;

pub fn exercise() {
    prompt("[test] Finite field elements (not equals)");

    let a = FiniteFieldElement::from((2, 31_u32));
    let b = FiniteFieldElement::from((2, 31_u32));
    let c = FiniteFieldElement::from((15, 31_u32));

    section("Assert a == b");

    message(&format!("a = {}", &a));
    message(&format!("b = {}", &b));

    assert_eq!(a, b);

    section("Assert a != c");

    message(&format!("a = {}", &a));
    message(&format!("c = {}", &c));

    assert_ne!(a, c);
}
