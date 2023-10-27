use std::io;
mod helpers;
use helpers::convert_to_u;

fn main(){

  let mut input_num1 = String::new();
  let mut input_num2 = String::new();
  println!("Enter the first number:");
  io::stdin().read_line(&mut input_num1).expect("Should be a number");
  println!("Enter the second number:");
  io::stdin().read_line(&mut input_num2).expect("Should be a number");
  let mut num1 = convert_to_u(&input_num1);
  let mut num2 = convert_to_u(&input_num2);
  #[allow(unused_assignments)]
  let mut rest: u128 = 0;

  if num1 < num2{
    rest = num1
  } else{
    rest = num2;
  }

  while rest > 1 {
    if num1 % rest == 0 && num2 % rest == 0 { break } 
    else { 
      rest = num1;
      num1 = num2 % num1;
      num2 = rest;
    };
  }

  println!("Greatest common divisor is: {rest}")

}