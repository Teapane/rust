fn main() {
    // mutable var
    let mut x = 5;
    println!("The Value of x is: {}", x);
    // x is now mutated
    x = 6;
    println!("The value of x is: {}", x);

    // consts never change, but must be typed
    const MAX: u32 = 100_000;

    println!("The value of max is: {}", MAX);

    // shadowing doesn't require mut,
    let y = 10;

    let y = y + 2;

    let y = y * 50000;

    println!("The value of y is: {}", y);

    // addition
    let sum = 5 + 10;

    println!("The value of sum is: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The value of multiplication is: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of y is: {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);

    let t = true;

    let f: bool = false; // with explicit type annotation
}
