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

    // booleans
    let t = true;
    println!("The value of t is: {}", t);

    let f: bool = false; // with explicit type annotation
    println!("The value of f is: {}", f);

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let(tup1, tup2, tup3) = tup;
    println!("The value of tup1 is: {}", tup1);
    println!("The value of tup2 is: {}", tup2);
    println!("The value of tup3 is: {}", tup3);

    // arrays
    let _array = [1, 2, 3, 4, 5];

    // we can use more than main!
    another_function();

    another_function_with_args(product);

    control_flow();
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_args(x: i32) {
    println!("The value of x is: {}", x);
}

fn control_flow() {
    let x = 3 ;

    if x < 5 {
        println!("The condition was false");
    } else {
        println!("The condition was true");
    }
}

