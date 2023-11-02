use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::finite_field_element::FiniteFieldElement;

pub fn exercise() {
    prompt("Numbers in the finite field of order 97 (multiplication and exponentiation)");

    section("95 * 45 * 31 = ...");

    let n_95 = FiniteFieldElement::from((95, 97_u32));
    let n_45 = FiniteFieldElement::from((45, 97_u32));
    let n_31 = FiniteFieldElement::from((31, 97_u32));
    let exp = n_95 * n_45 * n_31;

    show_display(&exp);

    section("17 * 13 * 19 * 44 = ...");

    let n_17 = FiniteFieldElement::from((17, 97_u32));
    let n_13 = FiniteFieldElement::from((13, 97_u32));
    let n_19 = FiniteFieldElement::from((19, 97_u32));
    let n_44 = FiniteFieldElement::from((44, 97_u32));
    let exp = n_17 * n_13 * n_19 * n_44;

    show_display(&exp);

    section("(12 ^ 7) * (77 ^ 49) = ...");

    let n_12 = FiniteFieldElement::from((12, 97_u32));
    let n_77 = FiniteFieldElement::from((77, 97_u32));
    let exp = (n_12.pow_i32(7)) * (n_77.pow_i32(49));

    show_display(&exp);
}
