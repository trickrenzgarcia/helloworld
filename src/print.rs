pub fn run() {
  println!("Hello world! from print.rs");

  // Basic formatting
  println!("{} is from {}", "number", "math");

  // Positional
  println!("{3}, {0}, {2}, {1}", "Ben", "Mark", "Joe", "Pat");

  // Name arguments
  println!("Hello my name is {name}, {age} years old", name = "Patrick", age = 21);

  // Basic Math
  println!("10 + 10 = {}", 10 + 10);
}