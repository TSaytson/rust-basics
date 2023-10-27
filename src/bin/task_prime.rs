mod helpers;
use std::io;

use helpers::convert_to_u;

fn is_prime(number: u128) -> bool {
    if number <= 1 {
        return false;
    };
    let limit = (number as f64).sqrt().ceil();
    for i in 2..=limit as u128 {
        if number % i == 0 {
            return false;
        };
    }
    return true;
}

fn main() {
  let mut input: String = String::new();
  while input.trim() != "s" {
    input.clear();
    println!("Enter a number or type 's' to exit:");
    io::stdin().read_line(&mut input).expect("Invalid input");
    let num = convert_to_u(&input);
    match is_prime(num) {
      true =>  println!("{num} is prime"),
      false => println!("{num} is not prime")
    }
  }
}
