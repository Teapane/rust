// import input/output from Standard Lib
use std::io;

// Entry point
fn main() {
    // print line
    println!("Guess the number!");
    println!("Please Input your guess.");

    // makes guess variable mutable
    let mut guess = String::new();

    // Reads users input. Expect is a safety check
    // &mut _references_ guess va
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    // print the users inputed val
    println!("You guessed: {}", guess);
}
