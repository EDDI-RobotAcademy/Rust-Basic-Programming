fn main() {
    let fibonacci_array = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];

    for element in &fibonacci_array {
        let result = match element {
            34 | 55 => "hit",
            _ => "miss",
        };

        if result == "hit" {
            println!("{}: {}", element, result);
        } else {
            println!("{}: {}", element, result);
        }
    }
}
