use std::io;
mod helpers;
use helpers::convert_to_int;

fn find_greater(vec: Vec<i32>) -> i32 {
  let mut max:i32 = vec[0];
    for i in vec {
      if i > max{
         max = i;
      }
    }
    max
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut input: String = String::new();

    while input.trim() != "s" {
        input.clear();
        println!("Enter a number or 's' to show results:");

        io::stdin()
            .read_line(&mut input)
            .expect("Should be a number");

        if input.trim() == "s" {
            break;
        } else {
            v.push(convert_to_int(&input))
        }
    }
    println!("greatest value is: {}", find_greater(v));
}
