mod helpers;
use std::io;

use helpers::convert_to_f;
fn main(){
  let mut v: Vec<f64> = Vec::new();
  let mut input:String = String::new();
  while input.trim() != "s" {
    input.clear();
    println!("Enter another number or type 's' to stop");
    io::stdin().read_line(&mut input).expect("Error");
    v.push(convert_to_f(&input));
  }
  let mut sum:f64 = 0.0;
  for item in &v{
    if item % 2.0 == 0.0{
      sum += item;
    }
  }

  println!("The sum of the even numbers is: {sum}");
}