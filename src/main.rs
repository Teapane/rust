// import input/output from Standard Lib
use std::io;
// random generator from rand crate
use rand::Rng;
use std::cmp::Ordering;

// Entry point
fn main() {
    // print line
    println!("Guess the number!");

    // immutable var
    // range is _inclusive_ on lower bound, but exclusive on upper bound
    let secret_number = rand::thread_rng().gen_range(1, 100001);
     loop {
        println!("Place your bets buckaroo.");

        // makes guess variable mutable
        let mut guess = String::new();

         // Reads users input. Expect is a safety check
         // &mut _references_ guess var
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // type check
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The limit does not exist!(Too low), 🥶"),
            Ordering::Greater => println!("Sure Jan!!(Too High), 📈"),
            Ordering::Equal => {
                println!("Congrats 💃🕺💃!");
                break;
            }
        }
    }
}
