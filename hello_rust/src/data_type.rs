//In Rust, the type of a variable is decided by the value you give it. Rust looks at the value and automatically chooses the right type

fn main () {
let my_num = 5;         // integer
let my_double = 5.99;   // float
let my_letter = 'D';    // character
let my_bool = true;     // boolean
let my_text = "Hello";  // string
}

//However, it is possible to explicitly tell Rust what type a value should be:

fn main () {
let my_num: i32 = 5;          // integer
let my_double: f64 = 5.99;    // float
let my_letter: char = 'D';    // character
let my_bool: bool = true;     // boolean
let my_text: &str = "Hello";  // string
}