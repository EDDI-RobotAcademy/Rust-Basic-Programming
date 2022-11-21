fn main() {
    let string_for_match = "test";
    let quote = "\
    Writing tests is very important in software programming.
    However, since the test is not omnipotent, you should not blindly trust it 100%.
    ";

    for (i, line) in quote.lines().enumerate() {
        if line.contains(string_for_match) {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
