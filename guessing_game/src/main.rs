// import standard library
use std::io;
// import rand, set dependencies in Cargo.toml
use rand::Rng;
use std::cmp::Ordering;

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

    // trans guess from String to u32
    // trim(): eliminate any whitespace at the beginning and end (includ \r \n)
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
