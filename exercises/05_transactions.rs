mod ch05_transactions;
mod util;

use util::run_exercise::*;
use ch05_transactions::*;

fn main() {
    match get_exercise_number() {
        1 => ex01_test_parse_version::exercise(),
        2 => ex02_test_parse_inputs::exercise(),
        3 => ex03_test_parse_outputs::exercise(),
        4 => ex04_test_parse_locktime::exercise(),
        5 => ex05_parse_transaction::exercise(),
        6 => ex06_test_fee::exercise(),
        0 => {
            other::test_transaction::run();
        },
        n => no_exercise_found(n),
    }
}
