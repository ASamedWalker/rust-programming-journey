use std::io::{self, Write};

fn main() {
  println!("What is your name? ");
  io::stdout().flush().unwrap();

  let mut name = String::new();
  io::stdin().read_line(&mut name).unwrap();

  println!("Hello, {name}, nice to meet you!");
}