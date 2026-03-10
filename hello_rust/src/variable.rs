fn main () {
let name = "Lerrize";
let age = 25;
println!("My name is {}, and my age is {}", name, age)
}

//By default, variables in Rust cannot be changed after they are created

/*If you want to change the value of a variable, you must use the mut keyword (which means mutable/changeable)*/
fn main () {
  let mut x = 5;
  println! ("{}",x);
  x = 7;
  println! ("{}", x)
}