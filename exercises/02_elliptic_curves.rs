mod ch02_elliptic_curves;
mod util;

use util::run_exercise::*;
use ch02_elliptic_curves::*;

fn main() {
    match get_exercise_number() {
        1 => ex01_points_on_curve::exercise(),
        2 => ex02_test_not_equal::exercise(),
        3 => ex03_test_add_1::exercise(),
        4 => ex04_point_addition_calculable_slope::exercise(),
        5 => ex05_test_add_2::exercise(),
        6 => ex06_point_addition_on_same_point::exercise(),
        7 => ex07_test_add_3::exercise(),
        n => no_exercise_found(n),
    }
}
