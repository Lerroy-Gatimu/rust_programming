/*loop
loop is the simplest of Rust's three loop types.

It will run forever unless you tell it to stop:

loop {
  println!("This will repeat forever!");
}*/

fn main() {
    let mut num = 0;
    loop {
        println!("Hey");
        if num == 3 {
            break;
        }
        num += 1;
    }
}

/*Return a Value
You can also return a value from a loop using break with a value.

This lets you save the result of the loop into a variable:
Note: When you save the result of a loop into a variable, you must put a semicolon (;) at the end.

Next: Learn how to use while loops to repeat code while a condition is true.

Example*/
fn main() {
    let mut num = 0;

    let va = loop {
        println!("Hey");

        if num == 3 {
            break num;
        }

        num += 1;
    };

    println!("The loop stopped at: {}", va);
}

/*The while Loop
The while loop runs as long as a condition is true.
The while loop checks the condition before each loop, so if the condition is false at the start, the loop will not run at all
Example*/
fn main() {
    let mut count = 1;

    while count <= 5 {
        println!("Count: {}", count);
        count += 1;
    }
}

/*Stop a While Loop
You can stop a while loop when you want by using break:

Example */

fn main() {
    let mut num = 1;

    while num <= 10 {
        if num == 6 {
            break;
        }
        println!("Number: {}", num);
        num += 1;
    }
}

/*Skip a Value
You can skip a value by using the continue statement:

Example*/
fn main() {
    let mut num = 1;

    while num <= 10 {
        if num == 6 {
            num += 1;
            continue;
        }

        println!("Number: {}", num);
        num += 1;
    }
}

/*
The for Loop
When you know exactly how many times you want to loop through a block of code, use the for loop together with the in keyword, instead of a while loop:

fn main () {

}
Example*/
fn main() {
    for i in 1..6 {
        println!("i is: {}", i);
    }
}

/*
This prints numbers from 1 to 5.

Note: 1..6 means from 1 up to (but not including) 6.

Note: Rust handles the counter variable (i) automatically, unlike many other programming languages. You don't need to declare or increment it manually.
If you want to include the last number, use ..= (two dots and an equals sign):
*/

fn main() {
    for i in 1..=6 {
        println!("i is: {}", i);
    }
}

/*
Break and Continue
Just like other loops, you can use break to stop the loop and continue to skip a value:
*/
fn main() {
    for i in 1..=10 {
        if i == 3 {
            continue; // skip 3
        }
        if i == 5 {
            break; // stop before printing 5
        }
        println!("i is: {}", i);
    }
}

fn main (){
  for i in 1..=8 {
    if i == 3 {
      continue;
    }
    if i == 6 {
      break;
    }
    println! ("Hey {}",i)
  }
}
