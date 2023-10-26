use std::io;
mod inputs;
use inputs::convert_to_int;

fn main(){

  let mut input_factorial = String::new();
  println!("Enter the factorial: ");
  io::stdin().read_line(&mut input_factorial).expect("Should be a number");
  let mut factorial:u128 = 1;
  
  let mut factorial_int: u128 = convert_to_int(&input_factorial);
  
  while factorial_int > 34{
    input_factorial.clear();
    println!("Enter a factorial less than 34");
    io::stdin().read_line(&mut input_factorial).expect("Should be less than 34");
    factorial_int = convert_to_int(&input_factorial);
  }

  while factorial_int > 1 {
    factorial *= factorial_int;
    factorial_int-=1;
  }

  println!("Factorial result is: {factorial}");

}