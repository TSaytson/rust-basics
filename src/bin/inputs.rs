use std::io;

pub(crate) fn convert_to_int(input: &str) -> u128 {
    return input.trim().parse::<u128>().unwrap();
}
#[allow(dead_code)]
fn main() {
    let mut num1: String = String::new();
    let mut num2: String = String::new();

    print!("Type the first number: \n");
    io::stdin()
        .read_line(&mut num1)
        .expect("Should be a number");
    print!("Type the second number: \n");
    io::stdin()
        .read_line(&mut num2)
        .expect("Should be a number");

    if convert_to_int(&num1) < convert_to_int(&num2) {
        print!("{} < {}", num1.trim(), num2.trim());
    } else if convert_to_int(&num1) == convert_to_int(&num2)  {
        print!("{} = {}", num1.trim(), num2.trim());
    } else {
        print!("{} > {}", num1.trim(), num2.trim())
    }
}
