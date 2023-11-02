use std::env::args;

pub fn get_exercise_number() -> i32 {
    args()
        .nth(1)
        .expect("Exercise number must be given as an integer argument")
        .parse::<i32>()
        .expect("Could not parse argument")
}

pub fn no_exercise_found(n: i32) {
    panic!("Exercise {} not found", n);
}
