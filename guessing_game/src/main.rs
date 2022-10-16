// import standard library
use std::io;
// import rand, set dependencies in Cargo.toml
use rand::Rng;

// entry point
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // rand number between (1, 100)
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {secret_number}");
    
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
