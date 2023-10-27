mod helpers;
use std::io;

use helpers::convert_to_u;

fn main(){
  let mut input:String = String::new();
  println!("Enter a number for multiplication table:");
  io::stdin().read_line(&mut input).expect("Error");
  let number = convert_to_u(&input);

  for i in 1..=10{
    println!("{i}x{number} = {}", i*number);
  }
}