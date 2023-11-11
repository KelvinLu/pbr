mod ch04_serialization;
mod util;

use util::run_exercise::*;
use ch04_serialization::*;

fn main() {
    match get_exercise_number() {
        1 => ex01_uncompressed_sec_format::exercise(),
        2 => ex02_compressed_sec_format::exercise(),
        3 => ex03_der_format::exercise(),
        4 => ex04_base58::exercise(),
        5 => ex05_bitcoin_address::exercise(),
        6 => ex06_bitcoin_wif::exercise(),
        9 => ex09_testnet_address::exercise(),
        0 => {
            other::test_sec_format::run();
            other::test_der_format::run();
            other::test_base58::run();
            other::test_addresses::run();
            other::test_wif::run();
        }
        n => no_exercise_found(n),
    }
}
