use std::ops::{Add};
use std::time::{Duration};

fn generic_add<T: Add<Output = T>>(num1: T, num2: T) -> T {
    num1 + num2
}

fn main() {
    let floats = generic_add(3.7, 7.7);
    let ints = generic_add(73, 33);
    let durations = generic_add(
        Duration::new(3, 0),
        Duration::new(7, 0)
    );

    println!("floats: {}", floats);
    println!("ints: {}", ints);
    println!("durations: {:?}", durations);
}