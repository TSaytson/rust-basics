use std::io;
mod helpers;
use helpers::convert_to_f;

fn average_calc(vec: Vec<f32>) -> f32{
  let average:f32 = vec.iter().sum();
  return average/((vec.len()-1) as f32);
}

fn main(){
  let mut input:String = String::new();
  let mut v: Vec<f32> = Vec::new();
  while input.trim() != "s" {
    input.clear();
    println!("Enter the grade or type 's' to stop");
    io::stdin().read_line(&mut input).expect("Error");
    v.push(convert_to_f(&input) as f32);
  }
  println!("The grades average is: {:.2}", average_calc(v));
}