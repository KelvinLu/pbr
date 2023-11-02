use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use math::finite_field_element::FiniteFieldElement;

pub fn exercise() {
    prompt("Fermat's little theorem");

    message("Consider a finite field with a prime order 'p' ...");
    message("... where every element is exponentiated by 'p-1'.");
    message("{1 ^ (p-1), 2 ^ (p-1), ..., (p-1) ^ (p-1)}");

    let primes: [u32; 5] = [7, 11, 17, 31, 43];

    for p in primes {
        section(&format!("p = {}", p));

        let series: Vec<FiniteFieldElement> = (0..p)
            .map(|n| FiniteFieldElement::from((n, p)))
            .collect();

        show_items(&series.iter().map(|n| format!("{} ^ {}", n, p - 1)).collect());
        message("...");
        show_items(&series.iter().map(|n| n.pow_u32(p - 1)).collect());
    }
}
