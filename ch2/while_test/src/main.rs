use std::time::{Instant};

fn main() {
    let mut i: i32 = 0;
    let start = Instant::now();

    while i < 100 {
        println!("{}", i);
        i += 1;
    }

    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
