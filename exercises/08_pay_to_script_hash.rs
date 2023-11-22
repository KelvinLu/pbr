mod ch08_pay_to_script_hash;
mod util;

use util::run_exercise::*;
use ch08_pay_to_script_hash::*;

fn main() {
    match get_exercise_number() {
        1 => ex01_test_op_checkmultisig::exercise(),
        2 => ex02_test_p2pkh_address::exercise(),
        3 => ex03_test_p2sh_address::exercise(),
        4 => ex04_validate_p2sh_transaction::exercise(),
        5 => ex05_test_verify_p2sh::exercise(),
        0 => {
            other::test_sign_input_p2sh::run();
        }
        n => no_exercise_found(n),
    }
}
