fn main() {
    let num = 77;
    let address = &num;
    let num2 = num + *address;

    println!("num + *address = {}", num2);
}
