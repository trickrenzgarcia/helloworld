pub fn run() {
  let name = "Pat";
  let mut age = 21;

  println!("My name is {} and I am {}", name, age);
  age = 22;
  println!("My name is {} and I am {}", name, age);

  // define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // assign multiple variable
  let ( myName, myAge ) = ("Job", 26);
  println!("{} is {}", myName, myAge);
}