use std::io;

fn main() {
  const RADIX: u32 = 10;
  let mut input: String = String::new();

  while input.trim() != "s"{
    input.clear();
    println!("Enter a number or type 's' to exit:");
    io::stdin()
        .read_line(&mut input)
        .expect("Should be a number");

    println!("Sum of the digits are: {}", 
      input.chars().map(|c| c.to_digit(RADIX).unwrap_or(0)).sum::<u32>());
  }
    
}