/*
Constant variables are used to store values that never change.
Unlike regular variables, constants must be defined with a type (e.g. i32 or char).
To create a constant, use the const keyword, followed by the name, type, and value
Another thing about constants, is that it is considered good practice to declare them with uppercase.
It is not required, but useful for code readability and common for Rust programmers
*/

fn main () {
  const NAME: &str = "Lerrizeeee";
  print! ("{}", NAME);
}

fn main() {
  let mut x = 10;
  println!("Start: {}", x);

  x += 5;
  println!("After += 5: {}", x);

  x -= 2;
  println!("After -= 2: {}", x);

  x *= 2;
  println!("After *= 2: {}", x);

  x /= 3;
  println!("After /= 3: {}", x);

  x %= 4;
  println!("After %= 4: {}", x);
}

fn main() {
  let logged_in = true;
  let is_admin = false;

  println!("Is regular user: {}", logged_in && !is_admin);
  println!("Has any access: {}", logged_in || is_admin);
  println!("Not logged in: {}", !logged_in);
}

