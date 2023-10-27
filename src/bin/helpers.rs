#![allow(dead_code)]
use std::io;
pub fn get_input(msg: &str, input: &mut String){
  while input.trim() != "s" {
    input.clear();
    println!("{msg} or type 's' to stop");
    io::stdin().read_line(input).expect("Should be a string");
  }
}
pub fn convert_to_int(input: &str) -> i32 {
    return input.trim().parse::<i32>().unwrap_or(0);
}
pub fn convert_to_f(input: &str) -> f64 {
    return input.trim().parse::<f64>().unwrap_or(0.0);
}
pub fn convert_to_u(input: &str) -> u128 {
    return input.trim().parse::<u128>().unwrap_or(0);
}

fn main(){}