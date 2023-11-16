mod ch06_script;
mod util;

use util::run_exercise::*;
use ch06_script::*;

fn main() {
    match get_exercise_number() {
        1 => ex01_test_op_hash160::exercise(),
        2 => ex02_test_op_checksig::exercise(),
        3 => ex03_unlock_script::exercise(),
        4 => ex04_puzzle_script::exercise(),
        0 => {
            other::test_data_element_as_number::run();
            other::test_data_element_length::run();
            other::test_coalesce_data_opcode::run();
            other::test_script_construction::run();
        },
        n => no_exercise_found(n),
    }
}
