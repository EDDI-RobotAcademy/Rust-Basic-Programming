fn add_based_on_lifecycle<'num1, 'num2>(num1: &'num1 i32, num2: &'num2 i32) -> i32 {
    *num1 + *num2
}

fn main() {
    let num1 = 10;
    let num2 = 20;
    let res = add_based_on_lifecycle(&num1, &num2);

    println!("result: {}", res);
}
