mod ch03_elliptic_curve_cryptography;
mod util;

use util::run_exercise::*;
use ch03_elliptic_curve_cryptography::*;

fn main() {
    match get_exercise_number() {
        1 => ex01_points_on_curve::exercise(),
        2 => ex02_point_addition::exercise(),
        3 => ex03_test_add::exercise(),
        4 => ex04_point_scalar_multiplication::exercise(),
        5 => ex05_generator_point::exercise(),
        6 => ex06_verify_message::exercise(),
        7 => ex07_sign_message::exercise(),
        0 => {
            other::test_secp256k1_signature::run();
            other::rfc_6979::run();
            other::public_key_recovery::run();
            other::private_key_recovery::run();
            other::signing_nonsense::run();
        }
        n => no_exercise_found(n),
    }
}
