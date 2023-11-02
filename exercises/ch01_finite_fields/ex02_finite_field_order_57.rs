use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::finite_field_element::FiniteFieldElement;

pub fn exercise() {
    prompt("Numbers in the finite field of order 57 (addition and subtraction)");

    section("44 + 33 = ...");

    let n_44 = FiniteFieldElement::from((44, 57_u32));
    let n_33 = FiniteFieldElement::from((33, 57_u32));
    let exp = n_44 + n_33;

    show_display(&exp);

    section("9 - 29 = ...");

    let n_9 = FiniteFieldElement::from((9, 57_u32));
    let n_29 = FiniteFieldElement::from((29, 57_u32));
    let exp = n_9 - n_29;

    show_display(&exp);

    section("17 + 42 + 49 = ...");

    let n_17 = FiniteFieldElement::from((17, 57_u32));
    let n_42 = FiniteFieldElement::from((42, 57_u32));
    let n_49 = FiniteFieldElement::from((49, 57_u32));
    let exp = n_17 + n_42 + n_49;

    show_display(&exp);

    section("52 - 30 - 38 = ...");

    let n_52 = FiniteFieldElement::from((52, 57_u32));
    let n_30 = FiniteFieldElement::from((30, 57_u32));
    let n_38 = FiniteFieldElement::from((38, 57_u32));
    let exp = n_52 - n_30 - n_38;

    show_display(&exp);
}
