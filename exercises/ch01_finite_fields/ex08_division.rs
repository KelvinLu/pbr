use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::finite_field_element::FiniteFieldElement;

pub fn exercise() {
    prompt("Numbers in the finite field of order 31 (division)");

    section("3 / 24 = ...");

    let n_3 = FiniteFieldElement::from((3, 31_u32));
    let n_24 = FiniteFieldElement::from((24, 31_u32));
    let exp = n_3 / n_24;

    show_display(&exp);

    section("17 ^ -3 = ...");

    let n_17 = FiniteFieldElement::from((17, 31_u32));
    let exp = n_17.pow_i32(-3);

    show_display(&exp);

    section("(4 ^ -4) * 11 = ...");

    let n_4 = FiniteFieldElement::from((4, 31_u32));
    let n_11 = FiniteFieldElement::from((11, 31_u32));
    let exp = n_4.pow_i32(-4) * n_11;

    show_display(&exp);
}
