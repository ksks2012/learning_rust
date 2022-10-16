// import standard library
use std::io;

// entry point
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // create a variable with mut (mutable)
    // '::' means associated function new() is in String
    let mut guess = String::new();

    // obtain user input from terminal
    io::stdin()
        .read_line(&mut guess)
        // error-handling
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
