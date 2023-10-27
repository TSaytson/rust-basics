use std::io;
mod helpers;
use helpers::convert_to_u;

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

    if convert_to_u(&num1) < convert_to_u(&num2) {
        print!("{} < {}", num1.trim(), num2.trim());
    } else if convert_to_u(&num1) == convert_to_u(&num2)  {
        print!("{} = {}", num1.trim(), num2.trim());
    } else {
        print!("{} > {}", num1.trim(), num2.trim())
    }
}
