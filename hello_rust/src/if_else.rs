fn main() {
    let age = 18;
    if age >= 18 {
        println!("This is an adult");
    } else {
        print!("This is a minor");
    }
}

fn main() {
    let score = 85;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }
}

/*
In Rust, if...else can also be used as an expression.
This means you can assign the result of an if to a variable
*/
fn main() {
    let time = 20;
    let greeting = if time < 18 {
        "Good day."
    } else {
        "Good evening."
    };
    println!("{}", greeting);
}

/*Simplified Syntax
If each block only contains a single expression, you can write it in a shorter way on one line:

Example:*/

fn main () {
let time = 20;
let greeting = if time < 18 { "Good day." } else { "Good evening." };
println!("{}", greeting);
}

//Tip: This works similarly to the ternary operator (condition ? value1 : value2) in languages like Java or C. However, Rust does not have a ternary operator, but using if...else as an expression gives you the same effect.

/*Don't Mix Types
Note: The value from if and else must be the same type, like two pieces of text or two numbers (in the example above, both are strings).

When you mix types, like a string and an integer, you'll get an error*/
fn main() {
let number = 5;
let result = if number < 10 { "Too small" } else { 100 };
println!("{}", result);
}

/*Match
When you have many choices, using match is easier than writing lots of if...else.
match is used to select one of many code blocks to be executed
In Rust, every match statement must handle all possible values unless you include a wildcard pattern "_".
*/
fn main() {
  let day = 4;

  match day {
    1 => println!("Monday"),
    2 => println!("Tuesday"),
    3 => println!("Wednesday"),
    4 => println!("Thursday"),
    5 => println!("Friday"),
    6 => println!("Saturday"),
    7 => println!("Sunday"),
    _ => println!("Invalid day."),
  }
}

/*You can match multiple values at once using the | operator (OR)

Example*/
fn main() {
  let day = 6;

  match day {
    1 | 2 | 3 | 4 | 5 => println!("Weekday"),
    6 | 7 => println!("Weekend"),
    _ => println!("Invalid day"),
  }
}

/*match with a Return Value
Just like if, match can also return a value:

This means you can save the result of a match into a variable:

Example*/
fn main() {
  let day = 4;

  let result = match day {
    1 => "Monday",
    2 => "Tuesday",
    3 => "Wednesday",
    4 => "Thursday",
    5 => "Friday",
    6 => "Saturday",
    7 => "Sunday",
    _ => "Invalid day.",
  };

  println!("{}", result);
}