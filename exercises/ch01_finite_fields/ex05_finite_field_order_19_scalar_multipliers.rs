use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::finite_field_element::FiniteFieldElement;

pub fn exercise() {
    prompt("Numbers in the finite field of order 19 (scalar multipliers)");

    message("{k * 0, k * 1, ..., k * 18}");

    for k in [1, 3, 7, 13, 18] {
        section(&format!("k = {}", k));

        let mut series: Vec<FiniteFieldElement> = (0..19)
            .map(|n| k * FiniteFieldElement::from((n, 19_u32)))
            .collect();

        message("with original ordering, after scalar multipliation ...");
        show_items(&series);

        series.sort();

        message("now sorted ...");
        show_items(&series);
    }
}
