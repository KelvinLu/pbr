mod ch07_transaction_validation_and_creation;
mod util;

use util::run_exercise::*;
use ch07_transaction_validation_and_creation::*;

fn main() {
    match get_exercise_number() {
        1 => ex01_test_sig_hash::exercise(),
        2 => ex02_test_verify_p2pkh::exercise(),
        3 => ex03_test_sign_input::exercise(),
        4 => ex04_testnet_faucet_1::exercise(),
        n => no_exercise_found(n),
    }
}
