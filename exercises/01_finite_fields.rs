mod ch01_finite_fields;
mod util;

use util::run_exercise::*;
use ch01_finite_fields::*;

fn main() {
    match get_exercise_number() {
        1 => ex01_test_not_equal::exercise(),
        2 => ex02_finite_field_order_57::exercise(),
        3 => ex03_test_subtraction::exercise(),
        4 => ex04_multiplication_and_exponentiation::exercise(),
        5 => ex05_finite_field_order_19_scalar_multipliers::exercise(),
        6 => ex06_test_multiplication::exercise(),
        7 => ex07_fermats_little_theorem::exercise(),
        8 => ex08_division::exercise(),
        9 => ex09_test_division::exercise(),
        0 => {
            other::test_addition::run();
            other::test_exponentiation::run();
        },
        n => no_exercise_found(n),
    }
}
